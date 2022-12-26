use std::path::Path;

use futures::TryStreamExt;
use chrono::{DateTime, Local};
use sqlx::{PgPool, Row};
use serde::Deserialize;
use id3::TagLike;

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub filename: Option<String>,
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

    pub async fn update(&self, db: &PgPool, new_song: &NewSong) -> Result<(), sqlx::Error> {
        sqlx::query(r#"
            UPDATE songs
            SET
                title      = $1,
                artist     = $2,
                album      = $3,
                duration   = $4,
                updated_at = NOW()
            WHERE id = $5;
        "#).
            bind(&new_song.title).
            bind(&new_song.artist).
            bind(&new_song.album).
            bind(&new_song.duration).
            bind(&self.id).
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
    pub filename: Option<String>,
}

impl NewSong {
    pub fn from_path(path: &Path) -> Result<Self, ::id3::Error> {
        let tag = ::id3::Tag::read_from_path(&path)?;
        let duration = ::mp3_duration::from_path(&path).
            map(|d| d.as_secs()).
            unwrap_or(0) as i32;

        Ok(Self {
            title: tag.title().map(|t| t.to_owned()).
                unwrap_or_else(|| format!("{}", path.display())),
            artist: tag.artist().map(String::from),
            album: tag.album().map(String::from),
            duration,
            filename: Some(path.file_name().unwrap().to_string_lossy().to_string()),
        })
    }

    pub async fn insert(&self, db: &PgPool) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(r#"
            INSERT INTO songs
            (title, artist, album, duration, created_at, updated_at, filename)
            VALUES
            ($1, $2, $3, $4, NOW(), NOW(), $5)
            RETURNING id;
        "#).
            bind(&self.title).
            bind(&self.artist).
            bind(&self.album).
            bind(&self.duration).
            bind(&self.filename).
            fetch_one(db);

        result.await?.try_get("id")
    }
}
