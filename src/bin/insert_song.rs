use std::env;
use std::path::PathBuf;

use dotenv::dotenv;
use sqlx::PgPool;

use spotiferris::models::*;

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).
        await.
        unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    for filename in env::args().skip(1) {
        let path = PathBuf::from(filename);
        let new_song = match NewSong::from_path(&path) {
            Ok(song) => song,
            Err(e) => {
                eprintln!("Skipping {}, error: {}", path.display(), e);
                continue;
            }
        };

        println!("Inserting: {:?}", new_song);
        new_song.insert(&db).await.unwrap();
    }
}
