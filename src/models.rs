use std::time::SystemTime;

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

impl Default for Song {
    fn default() -> Song {
        let now = SystemTime::now();

        Song {
            id: 0,
            title: String::from(""),
            artist: None,
            album: None,
            duration: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
