extern crate dotenv;

use dotenv::dotenv;
use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => println!("Loaded env file."),
        Err(_) => println!("Failed to load env file!"),
    }

    let asserts_dir = env::var("ASSERTS_DIR").unwrap_or_else(|_| "/app/asserts".to_owned());

    let app = Router::new()
        .nest_service("/", ServeDir::new(asserts_dir))
        .route("/protected", get(protected));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn protected(jar: CookieJar) -> impl IntoResponse {
    let jwt_cookie = match jar.get("jwt") {
        Some(cookie) => cookie,
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let api_client = reqwest::Client::builder().build().unwrap();

    let verify_token_body = serde_json::json!({
        "token": &jwt_cookie.value(),
    });

    let auth_hostname = env::var("AUTH_SERVICE_HOST").unwrap_or("0.0.0.0".to_owned());
    let url = format!("http://{}/verify-token", auth_hostname);

    let response = match api_client.post(&url).json(&verify_token_body).send().await {
        Ok(response) => response,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match response.status() {
        reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::BAD_REQUEST => {
            StatusCode::UNAUTHORIZED.into_response()
        }
        reqwest::StatusCode::OK => Json(ProtectedRouteResponse {
            img_url: "https://i.ibb.co/YP90j68/Light-Live-Bootcamp-Certificate.png".to_owned(),
        })
        .into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
pub struct ProtectedRouteResponse {
    pub img_url: String,
}
