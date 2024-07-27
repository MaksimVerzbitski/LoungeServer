use axum::{
    extract::{Path, Form, Extension},
    http::{StatusCode, header},
    response::{IntoResponse, Json},
};
use deadpool_postgres::{Pool, Client};
use serde_json::json;
use crate::models::user::{User, CreateUser, UpdateUser};
use serde::Deserialize;
use hyper::Response;

#[derive(Debug, Deserialize)]
pub struct CreateUserForm {
    pub telegram_id: i64,
    pub username: String,
    pub tokens: i32,
    pub referals: i32,
    pub friends: i64,
    pub active_chat: i32,
}



pub async fn create_user(
    Form(payload): Form<CreateUserForm>,
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    if let Err(err) = validate_username(&payload.username) {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": err })));
    }

    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    let check_statement = match client
        .prepare("SELECT id FROM users WHERE telegram_id = $1")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    let existing_users = match client.query(&check_statement, &[&payload.telegram_id]).await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to check existing users: {}", e) }))),
    };

    if !existing_users.is_empty() {
        return (StatusCode::CONFLICT, Json(json!({ "error": "User with this telegram_id already exists" })));
    }

    let statement = match client
        .prepare("INSERT INTO users (telegram_id, username, tokens, referals, friends, active_chat) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client
        .query(&statement, &[
            &payload.telegram_id,
            &payload.username,
            &payload.tokens,
            &payload.referals,
            &payload.friends,
            &payload.active_chat,
        ])
        .await
    {
        Ok(rows) => {
            let id: i32 = rows[0].get(0);
            (StatusCode::CREATED, Json(json!({ "id": id, "message": format!("User {} created successfully", payload.username) })))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create user: {}", e) })),
        ),
    }
}




pub async fn get_user(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    let statement = match client
        .prepare("SELECT id, telegram_id, username, tokens, referals, friends, active_chat FROM users WHERE id = $1")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client.query(&statement, &[&id]).await {
        Ok(rows) => {
            if rows.is_empty() {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))
            } else {
                let user = User {
                    id: rows[0].get(0),
                    telegram_id: rows[0].get(1),
                    username: rows[0].get(2),
                    tokens: rows[0].get(3),
                    referals: rows[0].get(4),
                    friends: rows[0].get(5),
                    active_chat: rows[0].get(6),
                };
                (StatusCode::OK, Json(json!(user)))
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch user: {}", e) })),
        ),
    }
}

pub async fn get_all_users(
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    let statement = match client
        .prepare("SELECT id, telegram_id, username, tokens, referals, friends, active_chat FROM users")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client.query(&statement, &[]).await {
        Ok(rows) => {
            let users: Vec<User> = rows.iter().map(|row| User {
                id: row.get(0),
                telegram_id: row.get(1),
                username: row.get(2),
                tokens: row.get(3),
                referals: row.get(4),
                friends: row.get(5),
                active_chat: row.get(6),
            }).collect();
            (StatusCode::OK, Json(json!(users)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch users: {}", e) })),
        ),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserForm {
    pub telegram_id: Option<i64>,
    pub username: Option<String>,
    pub tokens: Option<i32>,
    pub referals: Option<i32>,
    pub friends: Option<i64>,
    pub active_chat: Option<i32>,
}

pub async fn update_user(
    Path(id): Path<i32>,
    Form(payload): Form<UpdateUserForm>,
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    if let Some(username) = &payload.username {
        if let Err(err) = validate_username(username) {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": err })));
        }
    }

    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    // Check if the user exists
    let check_statement = match client
        .prepare("SELECT id, telegram_id, username, tokens, referals, friends, active_chat FROM users WHERE id = $1")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    let existing_users = match client.query(&check_statement, &[&id]).await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to check existing user: {}", e) }))),
    };

    if existing_users.is_empty() {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })));
    }

    let current_user = &existing_users[0];
    let updated_user = UpdateUser {
        id,
        telegram_id: payload.telegram_id.unwrap_or(current_user.get(1)),
        username: payload.username.clone().unwrap_or(current_user.get(2)),
        tokens: payload.tokens.unwrap_or(current_user.get(3)),
        referals: payload.referals.unwrap_or(current_user.get(4)),
        friends: payload.friends.unwrap_or(current_user.get(5)),
        active_chat: payload.active_chat.unwrap_or(current_user.get(6)),
    };

    // Update user details
    let statement = match client
        .prepare("UPDATE users SET telegram_id = $1, username = $2, tokens = $3, referals = $4, friends = $5, active_chat = $6 WHERE id = $7 RETURNING id, telegram_id, username, tokens, referals, friends, active_chat")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client
        .query(&statement, &[
            &updated_user.telegram_id,
            &updated_user.username,
            &updated_user.tokens,
            &updated_user.referals,
            &updated_user.friends,
            &updated_user.active_chat,
            &id,
        ])
        .await
    {
        Ok(rows) => {
            let updated_user = User {
                id: rows[0].get(0),
                telegram_id: rows[0].get(1),
                username: rows[0].get(2),
                tokens: rows[0].get(3),
                referals: rows[0].get(4),
                friends: rows[0].get(5),
                active_chat: rows[0].get(6),
            };
            (StatusCode::OK, Json(json!({ "message": "User updated successfully", "user": updated_user })))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update user: {}", e) })),
        ),
    }
}



pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    // Check if the user exists
    let check_statement = match client
        .prepare("SELECT id FROM users WHERE id = $1")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    let existing_users = match client.query(&check_statement, &[&id]).await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to check existing user: {}", e) }))),
    };

    if existing_users.is_empty() {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })));
    }

    // Delete user
    let statement = match client
        .prepare("DELETE FROM users WHERE id = $1")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client.execute(&statement, &[&id]).await {
        Ok(result) => {
            if result == 0 {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))
            } else {
                (StatusCode::OK, Json(json!({ "message": "User deleted successfully" })))
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to delete user: {}", e) })),
        ),
    }
}


pub async fn delete_all_users(
    Extension(pool): Extension<Pool>,
) -> impl IntoResponse {
    let client: Client = match pool.get().await {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to get client: {}", e) }))),
    };

    let statement = match client
        .prepare("DELETE FROM users")
        .await
    {
        Ok(statement) => statement,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to prepare statement: {}", e) }))),
    };

    match client.execute(&statement, &[]).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "All users deleted successfully" }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to delete all users: {}", e) })),
        ),
    }
}



pub async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let message = format!("Hello, {}!", name);
    (StatusCode::OK, Json(json!({ "message": message })))
}

pub async fn root(Extension(pool): Extension<Pool>) -> impl IntoResponse {
    let response_body = match pool.get().await {
        Ok(_) => {
            println!("Database connection is healthy\n");
            "<html><head></head>
            <body style='background-color: black; color: white; text-align: center;'>
                <div style='display: inline-block; border: 2px solid white; padding: 20px;'>
                    <div style='border-bottom: 2px solid white; padding-bottom: 10px;'>
                        <span style='font-weight: bold;'>Welcome to the LoungeCode Server!</span>
                    </div>
                    <div style='border-bottom: 2px solid white; padding: 10px 0;'>
                        <span>Database connection is healthy</span>
                    </div>
                    <div style='padding-top: 10px;'>
                        <span>Visit <a href=\"http://127.0.0.1:3000/user\" style='color: lightblue;'>User Management</a></span>
                    </div>
                </div>
            </body></html>".to_string()
        }
        Err(_) => {
            println!("Failed to connect to the database\n");
            "<html><head></head>
            <body style='background-color: black; color: white; text-align: center;'>
                <div style='display: inline-block; border: 2px solid white; padding: 20px;'>
                    <div style='border-bottom: 2px solid white; padding-bottom: 10px;'>
                        <span style='font-weight: bold;'>Welcome to the LoungeCode Server!</span>
                    </div>
                    <div style='border-bottom: 2px solid white; padding: 10px 0;'>
                        <span>Failed to connect to the database</span>
                    </div>
                    <div style='padding-top: 10px;'>
                        <span>Visit <a href=\"http://127.0.0.1:3000/user\" style='color: lightblue;'>User Management</a></span>
                    </div>
                </div>
            </body></html>".to_string()
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(response_body)
        .unwrap()
}

fn validate_username(username: &str) -> Result<(), String> {
    let username_len = username.len();
    if username_len > 11 {
        return Err("Username cannot be longer than 11 characters".to_string());
    }

    let mut chars = username.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_alphabetic() {
            return Err("The first character must be a letter".to_string());
        }

        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '-' && c != '\'' {
                return Err("Username can only contain letters, digits, and special characters (_-')".to_string());
            }
        }
    } else {
        return Err("Username cannot be empty".to_string());
    }

    Ok(())
}
