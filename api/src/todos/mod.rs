use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::prisma::{self, todo, PrismaClient};
use prisma_client_rust::NewClientError;

pub fn create_route() -> Router {
    Router::new().route("/todos", get(get_todos()) /*.post(create_todos()) */)
    // .route(
    //     "/todos/:id",
    //     get(get_todo_item())
    //         .put(uapdate_todos())
    //         .delete(delete_todos()),
    // )
}

async fn get_todos() {
    let client: Result<PrismaClient, NewClientError> = prisma::new_client().await;
    // client
}
// async fn get_todo_item() {}
// async fn create_todos() {}
// async fn uapdate_todos() {}
// async fn delete_todos() {}
