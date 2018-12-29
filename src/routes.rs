use gotham::handler::IntoResponse;
use gotham::state::State;

pub mod songs {
    use super::*;

    pub fn index(state: State) -> (State, impl IntoResponse) {
        (state, "songs index")
    }

    pub fn show(state: State) -> (State, impl IntoResponse) {
        (state, "song show")
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
