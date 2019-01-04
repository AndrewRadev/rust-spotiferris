use spotiferris::*;

pub fn main() {
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routing::router())
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
        use std::time::SystemTime;

        #[test]
        fn get_single_song() {
            use diesel::prelude::*;
            use crate::schema::songs;
            use crate::models::*;

            let db = establish_db_connection();
            let now = SystemTime::now();
            let new_song = NewSong {
                title: "The Sad Song",
                artist: Some("Johnny Cash"),
                album: None, duration: 0, created_at: now, updated_at: now,
            };

            let created_song = diesel::insert_into(songs::table)
                .values(&new_song)
                .get_result::<Song>(&db)
                .expect("Error saving new song");

            let test_server = TestServer::new(router()).unwrap();
            let response = test_server
                .client()
                .get(format!("http://localhost/songs/{}", created_song.id))
                .perform()
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let raw_body = response.read_body().unwrap();
            let body = String::from_utf8_lossy(&raw_body);

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
