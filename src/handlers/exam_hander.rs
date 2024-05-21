use crate::common::constants::RUST_WEB_DB;
use crate::common::response::ResVO;
use crate::config::AppState;
use crate::models::exam_mdl::ExamMdl;
use crate::ResponseResult;
use axum::extract::{Path, Query};
use axum::{Extension, Form, Json};
use mongodb::bson::doc;
use std::collections::HashMap;
use std::sync::Arc;

const EXAM_TABLE: &str = "exam";

/**
  json格式
*/
pub async fn insert(
    Extension(state): Extension<Arc<AppState>>,
    Json(exam): Json<ExamMdl>,
) -> ResponseResult<String> {
    state
        .db_pool
        .database(RUST_WEB_DB)
        .collection::<ExamMdl>(EXAM_TABLE)
        .insert_one(exam, None)
        .await
        .ok();
    Ok(Json(ResVO::<String>::success_without_data()))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Json(exam): Json<ExamMdl>,
) -> ResponseResult<String> {
    let update = mongodb::bson::to_document(&exam).unwrap();
    state
        .db_pool
        .database(RUST_WEB_DB)
        .collection::<ExamMdl>(EXAM_TABLE)
        .update_one(doc! {"_id": exam.id}, update, None)
        .await
        .ok();
    Ok(Json(ResVO::<String>::success_without_data()))
}
/**
 path路径形式
*/
pub async fn query_one(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> ResponseResult<ExamMdl> {
    let exam = state
        .db_pool
        .database(RUST_WEB_DB)
        .collection::<ExamMdl>(EXAM_TABLE)
        .find_one(doc! {"_id": id}, None)
        .await
        .unwrap();
    Ok(Json(ResVO::success(exam)))
}

/**
 form表单形式
*/
pub async fn query_exams(
    Extension(state): Extension<Arc<AppState>>,
    Form(query): Form<ExamMdl>,
) -> ResponseResult<String> {
    Ok(Json(ResVO::<String>::success_without_data()))
}

/**
 *参数提交
*/
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Query(arg): Query<HashMap<String, String>>,
) -> ResponseResult<String> {
    state
        .db_pool
        .database(RUST_WEB_DB)
        .collection::<ExamMdl>(EXAM_TABLE)
        .delete_one(doc! {"_id": arg.get("id")}, None)
        .await
        .unwrap();
    Ok(Json(ResVO::<String>::success_without_data()))
}
