use askama::Template;
use gotham::handler::IntoResponse;
use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Response, Body, StatusCode};

use crate::models::*;
use crate::establish_db_connection;

pub mod songs {
    use super::*;
    use diesel::prelude::*;
    use gotham_derive::*;
    use serde_derive::*;
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

    #[derive(Deserialize, StateData, StaticResponseExtender)]
    pub struct SongExtractor {
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

        let requested_id = SongExtractor::borrow_from(&state).id;
        let db = establish_db_connection();
        let record = match songs.filter(id.eq(requested_id)).first::<Song>(&db) {
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

    pub fn create(state: State) -> (State, impl IntoResponse) {
        (state, "song create")
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
