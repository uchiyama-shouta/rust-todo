use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub mod prisma;

// - `GET /todos`: return a JSON list of Todos.
// - `GET /todos:id`: return a JSON list of Todos.
// - `POST /todos`: create a new Todo.
// - `PUT /todos/:id`: update a specific Todo.
// - `DELETE /todos/:id`: delete a specific Todo.

async fn handler() -> &'static str {
    "Hello, World!"
}

const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    println!("http:localhost:{}", PORT);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
