use futures::TryStreamExt;
use chrono::{DateTime, Local};
use sqlx::{PgPool, Row};
use serde::Deserialize;

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Song {
    pub async fn find_all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let mut rows = sqlx::query_as::<_, Song>("SELECT * FROM songs").
            fetch(db);

        let mut songs = Vec::new();
        while let Some(song) = rows.try_next().await? {
            songs.push(song);
        }

        Ok(songs)
    }

    pub async fn find_one(db: &PgPool, id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Song>("SELECT * FROM songs WHERE id = $1").
            bind(id).
            fetch_one(db).await
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(r#"
            UPDATE songs
            SET
                title      = $1,
                artist     = $2,
                album      = $3,
                duration   = $4,
                updated_at = NOW();
        "#).
            bind(&self.title).
            bind(&self.artist).
            bind(&self.album).
            bind(&self.duration).
            execute(db).await?;

        Ok(())
    }

    pub async fn destroy(&self, db: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM songs WHERE id = $1;").
            bind(&self.id).
            execute(db).await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewSong {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: i32,
}

impl NewSong {
    pub async fn insert(&self, db: &PgPool) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(r#"
            INSERT INTO songs
            (title, artist, album, duration, created_at, updated_at)
            VALUES
            ($1, $2, $3, $4, NOW(), NOW())
            RETURNING id;
        "#).
            bind(&self.title).
            bind(&self.artist).
            bind(&self.album).
            bind(&self.duration).
            fetch_one(db);

        result.await?.try_get("id")
    }
}
