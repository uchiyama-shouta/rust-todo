use axum::{
    routing::{get, patch},
    Router,
};

use super::service;

// Router
pub fn create_route() -> Router {
    Router::new()
        .route("/", get(service::get_todos).post(service::create_todos))
        .route(
            "/:id",
            patch(service::update_todos).delete(service::delete_todos),
        )
}
