pub mod common;
pub mod db;
pub mod handlers;
pub mod models;
pub mod service;
pub mod utils;

pub mod router;

pub mod config;

pub mod interceptor;

use crate::common::errors::AppError;

use crate::common::response::ResVO;

use axum::Json;

pub type Result<T> = std::result::Result<T, AppError>;

pub type ResponseResult<T> = std::result::Result<Json<ResVO<T>>, AppError>;
