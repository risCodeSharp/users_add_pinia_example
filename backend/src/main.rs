use axum::{
    Error, Router,
    extract::{Json, State},
    http::{Method, },// header
    response::IntoResponse,
    routing,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    fn new(id: u32, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}
#[derive(Clone)]
struct AppState {
    pub users: Arc<RwLock<Vec<User>>>,
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut users = state.users.write().await;
    let new_user = User::new((users.len() + 1) as u32, &payload.name, &payload.email);
    users.push(new_user.clone());
    Json(new_user)
}

async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let _ = tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let users = state.users.read().await;

    Json(users.clone())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);
    let initial_user = vec![
        User::new(1, "abc", "abc@gmail.com"),
        User::new(2, "john_doe", "john@example.com"),
        User::new(3, "jane_smith", "jane@example.com"),
        User::new(4, "alex_wilson", "alex@example.com"),
        User::new(5, "emma_johnson", "emma@example.com"),
        User::new(6, "michael_brown", "michael@example.com"),
        User::new(7, "sarah_davis", "sarah@example.com"),
        User::new(8, "chris_miller", "chris@example.com"),
    ];
    let state = AppState {
        users: Arc::new(RwLock::new(initial_user)),
    };

    let router: Router = Router::new()
        .route("/users", routing::post(create_user))
        .route("/users", routing::get(get_users))
        .with_state(state)
        .layer(cors);

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Port is already been used!");

    println!("Listneing on: {addr}");

    axum::serve(listener, router)
        .await
        .expect("Error serving application");
    Ok(())
}
