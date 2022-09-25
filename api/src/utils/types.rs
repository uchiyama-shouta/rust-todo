use axum::{Extension, Json};

use super::errors;
use crate::prisma::PrismaClient;

pub type Database = Extension<std::sync::Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, errors::AppError>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
