use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: String,
    pub user_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_del: u8, //1 删除：0否
}

impl User {
    pub const TABLE: &'static str = "users";
}

// 用户注册请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserRequest {
    pub user_name: Option<String>,
    pub password: Option<String>,
}

// 用户信息展示
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoDisPlay {
    pub user_id: String,
    pub user_name: String,
}
