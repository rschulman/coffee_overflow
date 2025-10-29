use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    state: State<crate::AppState>,
    cookies: Cookies,
    Json(deets): Json<LoginCredentials>,
) -> Result<Json<LoginResponse>, (StatusCode, &'static str)> {
    // Salt and hash the submitted password
    let salted = format!("{}{}", deets.password, crate::SALT);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    // Find user in database
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(deets.username))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::FORBIDDEN, "Login failed"))?
        .ok_or((StatusCode::FORBIDDEN, "Login failed"))?;

    // Verify password matches
    if hashed_password == user.password {
        // Generate session token
        let session_token = Uuid::new_v4().to_string();

        // Set session cookie
        let mut cookie = Cookie::new("session", session_token.clone());
        cookie.set_path("/");
        cookie.set_http_only(true);
        cookies.add(cookie);

        Ok(Json(LoginResponse {
            token: session_token,
        }))
    } else {
        Err((StatusCode::FORBIDDEN, "Login failed"))
    }
}
