use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub telegram_id: i64,
    pub username: String,
    pub tokens: i32,
    pub referals: i32,
    pub friends: i64,
    pub active_chat: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub telegram_id: i64,
    pub username: String,
    pub tokens: i32,
    pub referals: i32,
    pub friends: i64,
    pub active_chat: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub telegram_id: i64,
    pub username: String,
    pub tokens: i32,
    pub referals: i32,
    pub friends: i64,
    pub active_chat: i32,
}
