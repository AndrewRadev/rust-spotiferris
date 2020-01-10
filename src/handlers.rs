use std::collections::HashMap;
use std::time::SystemTime;

use askama::Template;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoResponse, IntoHandlerError};
use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Response, Body, StatusCode};
use url::form_urlencoded;

use crate::models::*;
use crate::establish_db_connection;

pub mod songs {
    use super::*;
    use diesel::prelude::*;
    use serde::Deserialize;
    use gotham_derive::*;
    use gotham::state::FromState;

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

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct SongPath {
        id: i32,
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

    pub fn index(state: State) -> (State, impl IntoResponse) {
        use crate::schema::songs::dsl::*;

        let db = establish_db_connection();
        let records = songs.load::<Song>(&db).expect("Error loading songs");

        let template = IndexTemplate {
            title: "Song listing",
            songs: records.into_iter().map(|s| s.into()).collect(),
        };

        let response = render_template(&state, template);
        (state, response)
    }

    pub fn show(state: State) -> (State, impl IntoResponse) {
        use crate::schema::songs::dsl::*;

        let requested_id = SongPath::borrow_from(&state).id;
        let db = establish_db_connection();

        let query = songs.
            filter(id.eq(requested_id)).
            order(title);

        let record = match query.first::<Song>(&db) {
            Ok(record) => record,
            Err(_) => {
                let response = render_404(&state);
                return (state, response);
            }
        };
        let template: ShowTemplate = record.into();

        let response = render_template(&state, template);
        (state, response)
    }

    pub fn new(state: State) -> (State, impl IntoResponse) {
        let template = NewTemplate { title: "New Song" };
        let response = render_template(&state, template);

        (state, response)
    }

    pub fn create(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(|full_body| match full_body {
                Ok(valid_body) => {
                    let body_content = valid_body.into_bytes();
                    let form_data: HashMap<_, _> = form_urlencoded::parse(&body_content).
                        map(|(k, v)| (k.into_owned(), v.into_owned())).
                        collect();
                    let now = SystemTime::now();

                    let new_song = NewSong {
                        title: form_data.get("title").unwrap(),
                        artist: form_data.get("artist").map(String::as_str),
                        album: form_data.get("album").map(String::as_str),
                        duration: form_data.get("duration").and_then(|d| d.parse().ok()).unwrap_or(0),
                        created_at: now,
                        updated_at: now,
                    };

                    use crate::schema::songs::dsl::*;
                    let db = establish_db_connection();
                    let created_song = diesel::insert_into(songs)
                        .values(&new_song)
                        .get_result::<Song>(&db)
                        .expect("Error saving new song");
                    let song_url = format!("/songs/{}", created_song.id);

                    // The "temporary redirect" generates a 307, which confuses my browser
                    // let response = create_temporary_redirect(&state, song_url);

                    let mut response = create_empty_response(&state, StatusCode::FOUND);
                    response.
                        headers_mut().
                        insert(hyper::header::LOCATION, song_url.parse().unwrap());
                    future::ok((state, response))
                }
                Err(e) => future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }

    pub fn update(state: State) -> (State, impl IntoResponse) {
        (state, "song update")
    }

    pub fn delete(state: State) -> (State, impl IntoResponse) {
        (state, "song delete")
    }
}

pub mod api {
    pub mod songs {
        use gotham::state::State;
        use gotham::helpers::http::response::create_response;
        use hyper::{Body, Response, StatusCode};

        pub fn index(state: State) -> (State, Response<Body>) {
            let response = create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                "{}", // serde_json::to_string(&song).expect("serialized song"),
            );

            (state, response)
        }
    }
}

/// The response is either the rendered template, or a server error if something really goes wrong
fn render_template(state: &State, template: impl Template) -> Response<Body> {
    let response = match template.render() {
        Ok(content) => create_response(
            state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    response
}

fn render_404(state: &State) -> Response<Body> {
    create_response(
        state,
        StatusCode::NOT_FOUND,
        mime::TEXT_PLAIN,
        b"Not found" as &[u8]
    )
}
