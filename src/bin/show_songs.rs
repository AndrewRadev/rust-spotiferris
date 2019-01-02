use diesel::prelude::*;
use spotiferris::models::*;

fn main() {
    use spotiferris::schema::songs::dsl::*;
    use spotiferris::establish_db_connection;

    let connection = establish_db_connection();
    let results = songs
        .limit(5)
        .load::<Song>(&connection)
        .expect("Error loading songs");

    println!("Displaying {} songs", results.len());
    for song in results {
        println!("{} - {}", song.artist.unwrap_or("<Unknown>".to_string()), song.title);
    }
}
