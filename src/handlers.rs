use std::path::PathBuf;
use std::fs;
use std::io::Write;

use askama::Template;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use http::StatusCode;
use sqlx::PgPool;
use futures_util::TryStreamExt as _;
use log::warn;

fn render_template(template: impl Template) -> HttpResponse {
    match template.render() {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(e)       => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

pub async fn render_404(request: HttpRequest) -> Result<HttpResponse> {
    // use actix_web::Responder;
    // let file = NamedFile::open("static/404.html")?;
    // Ok(file.customize().with_status(StatusCode::NOT_FOUND).respond_to(&request))

    let file = NamedFile::open_async("static/404.html").await?;
    let mut response = file.into_response(&request);

    *response.status_mut() = StatusCode::NOT_FOUND;

    Ok(response)
}

pub mod songs {
    use super::*;
    use crate::models::{Song, NewSong};

    #[derive(Debug, Template)]
    #[template(path = "songs/index.html")]
    pub struct IndexTemplate {
        pub title: &'static str,
        pub songs: Vec<ShowTemplate>,
    }

    #[derive(Debug, Template)]
    #[template(path = "songs/show.html")]
    pub struct ShowTemplate {
        pub title: String,
        pub id: i32,
        pub album: String,
        pub display_title: String,
        pub duration: i32,
        pub created_at: String,
        pub updated_at: String,
        pub filename: Option<String>,
    }

    #[derive(Debug, Template)]
    #[template(path = "songs/edit.html")]
    pub struct EditTemplate {
        pub id: i32,
        pub title: String,
        pub album: String,
        pub artist: String,
        pub duration: i32,
    }

    #[derive(Debug, Template)]
    #[template(path = "songs/new.html")]
    pub struct NewTemplate {
        pub title: &'static str,
    }

    impl From<Song> for ShowTemplate {
        fn from(source: Song) -> Self {
            let artist = source.artist.unwrap_or("<Unknown>".to_string());
            let display_title = format!("{} - {}", artist, source.title);
            let filename = source.filename.
                map(|f| PathBuf::from(f).file_name().unwrap().to_str().unwrap().to_owned());

            ShowTemplate {
                id: source.id,
                title: display_title.clone(),
                album: source.album.unwrap_or("<Unknown>".to_string()),
                display_title,
                duration: source.duration,
                created_at: source.created_at.to_rfc2822(),
                updated_at: source.updated_at.to_rfc2822(),
                filename,
            }
        }
    }

    impl From<Song> for EditTemplate {
        fn from(source: Song) -> Self {
            EditTemplate {
                id:       source.id,
                title:    source.title.clone(),
                artist:   source.artist.unwrap_or_else(String::new),
                album:    source.album.unwrap_or_else(String::new),
                duration: source.duration,
            }
        }
    }

    pub async fn index(db: web::Data<PgPool>) -> HttpResponse {
        let records = Song::find_all(db.as_ref()).await.expect("Error loading songs");

        let template = IndexTemplate {
            title: "Song listing",
            songs: records.into_iter().map(|s| s.into()).collect(),
        };

        render_template(template)
    }

    pub async fn show(
        request: HttpRequest,
        db:      web::Data<PgPool>,
        path:    web::Path<i32>
    ) -> Result<HttpResponse> {
        let id = path.into_inner();

        match Song::find_one(&db, id).await {
            Ok(song) => Ok(render_template(ShowTemplate::from(song))),
            Err(_) => render_404(request).await,
        }
    }

    pub async fn edit(
        request: HttpRequest,
        db:      web::Data<PgPool>,
        path:    web::Path<i32>
    ) -> Result<HttpResponse> {
        let id = path.into_inner();

        match Song::find_one(&db, id).await {
            Ok(song) => Ok(render_template(EditTemplate::from(song))),
            Err(_) => render_404(request).await,
        }
    }

    pub async fn new() -> HttpResponse {
        render_template(NewTemplate { title: "Upload New Song" })
    }

    pub async fn create(
        db:          web::Data<PgPool>,
        mut payload: Multipart,
    ) -> Result<HttpResponse> {
        let mut last_id = 0;

        while let Some(mut field) = payload.try_next().await? {
            let filename = field.content_disposition().get_filename().
                map(|f| sanitize_filename::sanitize(f)).
                unwrap();
            let path = PathBuf::from("./public/uploads").join(filename);
            if path.extension().is_none() || path.extension().unwrap() != "mp3" {
                warn!("Skipping file '{}', not an mp3", path.display());
                continue;
            }

            // Blocking operations executed in a thread
            let path_clone = path.clone();
            let mut f = web::block(move || std::fs::File::create(&path_clone)).await??;
            while let Some(chunk) = field.try_next().await? {
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
            }

            let path_clone = path.clone();
            let new_song = match web::block(move || NewSong::from_path(&path_clone)).await? {
                Ok(song) => song,
                Err(e) => {
                    warn!("Skipping file '{}', couldn't get tags: {}", path.display(), e);
                    continue;
                },
            };

            last_id =
                match new_song.insert(&db).await {
                    Ok(id) => id,
                    Err(e) => return Ok(HttpResponse::BadRequest().body(format!("{:?}", e))),
                };
        }

        let location =
            if last_id > 0 {
                format!("/songs/{}", last_id)
            } else {
                String::from("/songs")
            };
        let response = HttpResponse::Found().
            insert_header(("Location", location)).
            finish();
        Ok(response)
    }

    pub async fn update(
        request: HttpRequest,
        db:      web::Data<PgPool>,
        path:    web::Path<i32>,
        form:    web::Form<NewSong>
    ) -> Result<HttpResponse> {
        let id = path.into_inner();
        let new_song = form.into_inner();

        let song = match Song::find_one(&db, id).await {
            Ok(record) => record,
            Err(_) => return render_404(request).await,
        };

        match song.update(&db, &new_song).await {
            Ok(_) => {
                let redirect = HttpResponse::Found().
                    insert_header(("Location", format!("/songs/{}", song.id))).
                    finish();
                Ok(redirect)
            },
            Err(_) => render_404(request).await,
        }
    }

    pub async fn delete(
        request: HttpRequest,
        db:      web::Data<PgPool>,
        path:    web::Path<i32>,
    ) -> Result<HttpResponse> {
        let id = path.into_inner();

        let song = match Song::find_one(&db, id).await {
            Ok(record) => record,
            Err(_) => return render_404(request).await,
        };

        if let Some(filename) = &song.filename {
            let path = PathBuf::from("./public/uploads").join(filename);
            fs::remove_file(&path).
                unwrap_or_else(|e| warn!("Couldn't delete file {}: {}", path.display(), e));
        }

        match song.destroy(&db).await {
            Ok(_) => {
                let redirect = HttpResponse::Found().
                    insert_header(("Location", "/songs")).
                    finish();
                Ok(redirect)
            },
            Err(_) => render_404(request).await,
        }
    }
}
