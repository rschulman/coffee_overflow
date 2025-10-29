use axum::{Router, routing::{post, get}, http::{Method, HeaderValue}};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

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

    // Get allowed origin from env or use default for development
    let frontend_origin = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    // Configure CORS securely - only allow specific frontend origin
    let cors = CorsLayer::new()
        .allow_origin(frontend_origin.parse::<HeaderValue>().expect("Invalid FRONTEND_URL"))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .expose_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600));

    let app = Router::new()
        .route("/login", post(login::login))
        .route("/register", post(register::register))
        .route("/user/details", get(user_details::user_details))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
