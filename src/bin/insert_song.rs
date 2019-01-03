use spotiferris::schema::songs;
use spotiferris::models::*;
use spotiferris::establish_db_connection;

use std::env;
use std::time::SystemTime;

use diesel::prelude::*;
use id3::Tag;
use mp3_duration;

fn main() {
    let connection = establish_db_connection();

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
        let now = SystemTime::now();

        let new_song = NewSong {
            title: tag.title().unwrap_or(&filename),
            artist: tag.artist(),
            album: tag.album(),
            duration,
            created_at: now,
            updated_at: now,
        };

        println!("{:?}", new_song);
        let created_song = diesel::insert_into(songs::table)
            .values(&new_song)
            .get_result::<Song>(&connection)
            .expect("Error saving new song");
        println!(" -> {:?}", created_song);
        println!("");
    }
}
