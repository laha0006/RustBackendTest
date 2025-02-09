use serde::Serialize;

#[derive(Serialize)]
pub struct Content {
    pub id: i32,
    pub title: String,
}

