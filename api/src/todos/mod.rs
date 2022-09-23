use axum::{
    extract::{Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};

use crate::prisma::{todo, PrismaClient};
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};

use serde::Deserialize;

type Database = Extension<std::sync::Arc<PrismaClient>>;
type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(get_todos).post(create_todos))
    // .route(
    //     "/:id",
    //     get(get_todo_item())
    //         .put(uapdate_todos())
    //         .delete(delete_todos()),
    // )
}

async fn get_todos(db: Database) -> AppJsonResult<Vec<todo::Data>> {
    let todos = db.todo().find_many(vec![]).exec().await?;
    Ok(Json::from(todos))
}

async fn create_todos(db: Database, Json(input): Json<CreateTodo>) -> AppJsonResult<todo::Data> {
    let data = db.todo().create(input.title, false, vec![]).exec().await?;
    Ok(Json::from(data))
}
// async fn uapdate_todos() {}
// async fn delete_todos() {}

enum AppError {
    PrismaError(QueryError),
    NotFound,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all differents errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
