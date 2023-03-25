use serde::Deserialize;


#[derive(Deserialize, Clone)]

pub struct CreateEntry{
    pub title: String,
    pub date: i64
}

#[derive(Deserialize,Clone)]

pub struct UpdateEntry{
    pub title: String
}