use dotenv::dotenv;
use sqlx::PgPool;

use spotiferris::models::{NewSong, Song};

async fn get_db() -> PgPool {
    dotenv().ok();

    let database_url = std::env::var("TEST_DATABASE_URL").
        expect("TEST_DATABASE_URL must be set");

    let db = PgPool::connect(&database_url).
        await.
        unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    sqlx::migrate!("./migrations").
        run(&db).
        await.
        unwrap();

    db
}

async fn clean_db(db: &PgPool) {
    sqlx::query("DELETE FROM songs").execute(db).await.unwrap();
}

#[actix_rt::test]
async fn test_song_insertion() {
    let db = get_db().await;
    clean_db(&db).await;

    let new_song = NewSong {
        title:    String::from("Atomyk Ebonpyre"),
        artist:   Some(String::from("Homestuck")),
        album:    Some(String::from("Homestuck Vol. 1-4")),
        duration: 249,
    };

    let id = new_song.insert(&db).await.unwrap();
    let song = Song::find_one(&db, id).await.unwrap();

    assert_eq!(song.title,    new_song.title);
    assert_eq!(song.artist,   new_song.artist);
    assert_eq!(song.album,    new_song.album);
    assert_eq!(song.duration, new_song.duration);
}

#[actix_rt::test]
async fn test_song_listing() {
    let db = get_db().await;
    clean_db(&db).await;

    let new_song = NewSong {
        title:    String::from("Set Theory"),
        artist:   Some(String::from("Carbon Based Patterns")),
        album:    Some(String::from("World of Sleepers")),
        duration: 300,
    };

    let id_1 = new_song.insert(&db).await.unwrap();
    let id_2 = new_song.insert(&db).await.unwrap();

    let songs = Song::find_all(&db).await.unwrap().
        into_iter().
        map(|s| s.id).collect::<Vec<_>>();

    assert_eq!(songs.len(), 2);
    assert!(songs.contains(&id_1));
    assert!(songs.contains(&id_2));
}
