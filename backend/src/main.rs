use axum::{Router, routing::{post, get}};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower_cookies::CookieManagerLayer;

mod login;
mod register;
mod user_details;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

pub const SALT: &str = "xfpgsctjdluhayufpdj8glbvhukrlstjbgdbljrl4p9fjlgdj476grj7hskul47gpj";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let app = Router::new()
        .route("/login", post(login::login))
        .route("/register", post(register::register))
        .route("/user/details", get(user_details::user_details))
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
