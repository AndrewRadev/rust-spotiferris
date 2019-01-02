#[macro_use]
extern crate diesel;

use std::env;
use gotham::handler::assets::FileOptions;
use gotham::router::Router;
use gotham::router::builder::*;

mod schema;
mod models;
mod routes;
mod api;

fn router() -> Router {
    build_simple_router(|route| {
        route.get_or_head("/").to(routes::songs::index);

        route.associate("/songs", |assoc| {
            assoc.get_or_head().to(routes::songs::index);
            assoc.post().to(routes::songs::create);
        });
        route.associate("/songs/:id", |assoc| {
            assoc.get().
                with_path_extractor::<routes::songs::SongExtractor>().
                to(routes::songs::show);
            assoc.put().
                with_path_extractor::<routes::songs::SongExtractor>().
                to(routes::songs::update);
            assoc.patch().
                with_path_extractor::<routes::songs::SongExtractor>().
                to(routes::songs::update);
            assoc.delete().
                with_path_extractor::<routes::songs::SongExtractor>().
                to(routes::songs::delete);
        });

        route.scope("/api", |route| {
            // Won't be used for the moment, but will be interesting later
            route.get("/songs").to(api::songs::index);
        });

        let mut assets_path = env::current_dir().unwrap();
        assets_path.push("public");
        route.get("*").to_dir(
            FileOptions::new(&assets_path)
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    })
}

pub fn main() {
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    macro_rules! assert_includes {
        ($haystack:expr, $needle:expr) => {
            {
                let message = format!("Expected {} to include {}", stringify!($haystack), stringify!($needle));
                assert!($haystack.contains($needle), message);
            }
        }
    }

    mod routes {
        use super::*;

        #[test]
        fn get_single_song() {
            let test_server = TestServer::new(router()).unwrap();
            let response = test_server
                .client()
                .get("http://localhost/songs/1")
                .perform()
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let raw_body = response.read_body().unwrap();
            let body = String::from_utf8_lossy(&raw_body);

            // TODO: create record, read it
            assert_includes!(body, "Johnny Cash")
        }

        #[test]
        fn get_missing_song() {
            let test_server = TestServer::new(router()).unwrap();
            let response = test_server
                .client()
                .get("http://localhost/songs/99")
                .perform()
                .unwrap();

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[test]
        fn create_new_song() {
            let test_server = TestServer::new(router()).unwrap();
            let response = test_server
                .client()
                .post("http://localhost/songs", "", mime::TEXT_PLAIN)
                .perform()
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let raw_body = response.read_body().unwrap();
            let body = String::from_utf8_lossy(&raw_body);

            assert_includes!(body, "song create")
        }
    }

    mod api {
        use super::*;

        #[test]
        fn get_list_of_songs() {
            let test_server = TestServer::new(router()).unwrap();
            let response = test_server
                .client()
                .get("http://localhost/api/songs")
                .perform()
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let raw_body = response.read_body().unwrap();
            let body = String::from_utf8_lossy(&raw_body);

            assert_eq!(body, "{}")
        }
    }
}
