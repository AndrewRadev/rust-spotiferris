use std::time::SystemTime;

#[derive(Debug)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug)]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: Option<&'a str>,
    pub album: Option<&'a str>,
    pub duration: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
