use std::time::SystemTime;
use crate::schema::songs;

#[derive(Debug, Queryable)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Insertable)]
#[table_name="songs"]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: Option<&'a str>,
    pub album: Option<&'a str>,
    pub duration: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
