use std::env;

use dotenv::dotenv;
use id3::Tag;
use mp3_duration;
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
        let tag = match Tag::read_from_path(&filename) {
            Ok(tag) => tag,
            Err(e) => {
                eprintln!("Skipping {}, error: {}", filename, e);
                continue;
            }
        };
        let duration = mp3_duration::from_path(&filename).
            map(|d| d.as_secs()).
            unwrap_or(0) as i32;

        let new_song = NewSong {
            title: tag.title().unwrap_or(&filename).to_owned(),
            artist: tag.artist().map(String::from),
            album: tag.album().map(String::from),
            duration,
        };

        println!("Inserting: {:?}", new_song);

        new_song.insert(&db).await.unwrap();
    }
}
