use askama::Template;
use actix_web::{Responder, HttpResponse};
use actix_web::error::InternalError;
use actix_files::NamedFile;
use http::StatusCode;

fn render_template(template: impl Template) -> impl Responder {
    template.render().
        map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn render_404() -> actix_web::Result<NamedFile> {
    let file = NamedFile::open("static/404.html")?;
    Ok(file.set_status_code(StatusCode::NOT_FOUND))
}

pub mod songs {
    use super::*;
    use crate::models::Song;
    // use crate::establish_db_connection;

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

    pub fn index() -> HttpResponse {
        HttpResponse::Ok().body("index")
    }

    pub fn show() -> HttpResponse {
        HttpResponse::Ok().body("show")
    }

    pub fn new() -> HttpResponse {
        HttpResponse::Ok().body("new")
    }

    pub fn create() -> HttpResponse {
        HttpResponse::Ok().body("create")
    }

    pub fn update() -> HttpResponse {
        HttpResponse::Ok().body("update")
    }

    pub fn delete() -> HttpResponse {
        HttpResponse::Ok().body("update")
    }
}
