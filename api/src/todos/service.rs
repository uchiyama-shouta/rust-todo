use axum::{
    extract::{Json, Path},
    http::StatusCode,
};

use serde::Deserialize;

use crate::{
    prisma::todo::{self, SetParam},
    utils::types::{AppJsonResult, AppResult, Database},
};

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub is_complete: Option<bool>,
}

// Get /
pub async fn get_todos(db: Database) -> AppJsonResult<Vec<todo::Data>> {
    let todos = db.todo().find_many(vec![]).exec().await?;
    Ok(Json::from(todos))
}

// Post /
pub async fn create_todos(
    db: Database,
    Json(input): Json<CreateTodo>,
) -> AppJsonResult<todo::Data> {
    let data = db.todo().create(input.title, false, vec![]).exec().await?;
    Ok(Json::from(data))
}

// Patch /:id
pub async fn update_todos(
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
pub async fn delete_todos(db: Database, Path(id): Path<i32>) -> AppResult<StatusCode> {
    db.todo().delete(todo::id::equals(id)).exec().await?;
    Ok(StatusCode::OK)
}
