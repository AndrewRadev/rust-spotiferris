table! {
    songs (id) {
        id -> Int4,
        title -> Varchar,
        artist -> Nullable<Varchar>,
        album -> Nullable<Varchar>,
        duration -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
