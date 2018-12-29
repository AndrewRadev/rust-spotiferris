use gotham::state::State;
use gotham::helpers::http::response::create_response;
use hyper::{Body, Response, StatusCode};

pub mod songs {
    use super::*;

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
