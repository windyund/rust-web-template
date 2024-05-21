use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::exam_hander;
use crate::{handlers::user_hander, interceptor::header_interceptor::verify_auth};

/**
 * @desc 总路由
 */
pub fn routers() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/user", user_routers())
        .nest("/api", api_routers())
}

async fn root() -> &'static str {
    "Hello, This is index  Page!"
}

/**
 * @desc 注册登录路由，不需登录
 */
pub fn user_routers() -> Router {
    Router::new()
        .route("/register", post(user_hander::user_register))
        .route("/login", post(user_hander::authorize))
}

/**
 * @desc API路由，需登录校验
 */
pub fn api_routers() -> Router {
    Router::new()
        .route("/get_me", get(user_hander::get_me))
        .route("/logout", get(user_hander::logout))
        .nest("/exam", exam_routers())
        .layer(from_fn(verify_auth))
}

/**
 * @desc 考试相关
 */
pub fn exam_routers() -> Router {
    Router::new()
        .route("/insert", post(exam_hander::insert))
        .route("/delete", delete(exam_hander::delete))
        .route("/update", put(exam_hander::update))
        .route("/queryOne/:id", get(exam_hander::query_one))
        .route("/query", post(exam_hander::query_exams))
}
