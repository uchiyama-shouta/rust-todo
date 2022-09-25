use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch},
    Extension, Router,
};

use crate::prisma::{
    todo::{self, SetParam},
    PrismaClient,
};
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

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    is_complete: Option<bool>,
}

// Router
pub fn create_route() -> Router {
    Router::new()
        .route("/", get(get_todos).post(create_todos))
        .route("/:id", patch(update_todos).delete(delete_todos))
}

// Get /
async fn get_todos(db: Database) -> AppJsonResult<Vec<todo::Data>> {
    let todos = db.todo().find_many(vec![]).exec().await?;
    Ok(Json::from(todos))
}

// Post /
async fn create_todos(db: Database, Json(input): Json<CreateTodo>) -> AppJsonResult<todo::Data> {
    let data = db.todo().create(input.title, false, vec![]).exec().await?;
    Ok(Json::from(data))
}

// Patch /:id
async fn update_todos(
    db: Database,
    Path(id): Path<i32>,
    Json(input): Json<UpdateTodo>,
) -> AppJsonResult<todo::Data> {
    let mut params: Vec<SetParam> = vec![];

    if let Some(title) = input.title {
        params.push(todo::title::set(title))
    }
    if let Some(is_complete) = input.is_complete {
        params.push(todo::is_complete::set(is_complete))
    }
    let data = db
        .todo()
        .update(todo::id::equals(id), params)
        .exec()
        .await?;
    Ok(Json::from(data))
}

// Delete /:id
async fn delete_todos(db: Database, Path(id): Path<i32>) -> AppResult<StatusCode> {
    db.todo().delete(todo::id::equals(id)).exec().await?;
    Ok(StatusCode::OK)
}

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
