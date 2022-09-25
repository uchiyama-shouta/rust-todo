use std::net::SocketAddr;
use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};
use dotenv::dotenv;

pub mod prisma;
mod todos;

// - `GET /todos`: return a JSON list of Todos.
// - `POST /todos`: create a new Todo.
// - `PATCH /todos/:id`: update a specific Todo.
// - `DELETE /todos/:id`: delete a specific Todo.

async fn handler() -> &'static str {
    "Hello, World!!!"
}

const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();
    let prisma_client = Arc::new(prisma::new_client().await.expect("DB not found!"));
    let app = Router::new()
        .route("/", get(handler))
        .nest("/todos", todos::create_route())
        .layer(Extension(prisma_client));
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    println!("http:localhost:{}", PORT);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
