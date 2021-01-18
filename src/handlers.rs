use askama::Template;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;
use http::StatusCode;
use sqlx::PgPool;

fn render_template(template: impl Template) -> HttpResponse {
    match template.render() {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(e)       => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

pub async fn render_404() -> actix_web::Result<NamedFile> {
    let file = NamedFile::open("static/404.html")?;
    Ok(file.set_status_code(StatusCode::NOT_FOUND))
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

            ShowTemplate {
                title: display_title.clone(),
                id: source.id,
                album: source.album.unwrap_or("<Unknown>".to_string()),
                display_title,
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
        db: web::Data<PgPool>,
        path: web::Path<i32>
    ) -> Result<HttpResponse> {
        let web::Path(id) = path;

        match Song::find_one(&db, id).await {
            Ok(song) => Ok(render_template(ShowTemplate::from(song))),
            Err(_) => render_404().await.and_then(|f| f.into_response(&request)),
        }
    }

    pub fn new() -> HttpResponse {
        render_template(NewTemplate { title: "New Song" })
    }

    pub async fn create(
        db:   web::Data<PgPool>,
        form: web::Form<NewSong>
    ) -> HttpResponse {
        match form.insert(&db).await {
            Ok(id) => {
                HttpResponse::Found().
                    set_header("Location", format!("/songs/{}", id)).
                    finish()
            },
            Err(e) => HttpResponse::BadRequest().body(format!("{:?}", e))
        }
    }

    pub fn update() -> HttpResponse {
        HttpResponse::Ok().body("update")
    }

    pub fn delete() -> HttpResponse {
        HttpResponse::Ok().body("update")
    }
}
