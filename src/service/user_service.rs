use crate::{
    common::constants::EXPIRE_HOURS,
    common::errors::AppError,
    db::axredis::get_redis_client,
    models::{
        auth::{AuthBody, AuthUser},
        user::{RegisterUserRequest, User, UserInfoDisPlay},
    },
    utils::{
        jwt::sign,
        rencrypt::{hash, verify},
    },
    Result,
};
use std::string::ToString;

use crate::common::constants::RUST_WEB_DB;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::Client;
use redis::{Commands, RedisResult};

const USER_TABLE: &str = "user";

// 用户登录
pub async fn user_auth(pool: &Client, payload: AuthUser) -> Result<AuthBody> {
    let user_info = find_user(pool, payload.user_name.as_ref().unwrap().as_str()).await?;

    let verify_res = verify(payload.password.as_ref().unwrap(), &user_info.password).unwrap();
    let token_key = format!("token:{}", &user_info.user_id);
    if verify_res {
        let rd_client = get_redis_client().unwrap();

        let mut con = rd_client.get_connection()?;

        // 查看是否已经登陆过
        let login_token_string: String =
            match redis::cmd("GET").arg(token_key.clone()).query(&mut con) {
                Ok(login_token) => {
                    tracing::debug!("user token is invalid!");
                    login_token
                }
                Err(err) => {
                    tracing::debug!("user token is expired! {:?}", err);
                    "".to_string()
                }
            };

        if !login_token_string.is_empty() {
            tracing::info!("you've already logged in");
            Ok(AuthBody::new(login_token_string))
        } else {
            tracing::info!("user logging");
            let token = sign(user_info.clone().user_id).unwrap();

            // con.set_ex(token_key,  token.clone(),EXPIRE_HOURS * 3600);
            let _: () = redis::cmd("SET")
                .arg(token_key)
                .arg(token.clone())
                .arg("EX")
                .arg(EXPIRE_HOURS * 3600)
                .query(&mut con)?;

            Ok(AuthBody::new(token))
        }
    } else {
        Err(AppError::forbidden())
    }
}

pub async fn create(pool: &Client, payload: RegisterUserRequest) -> Result<u64> {
    let query_user_count = user_count(pool, payload.user_name.as_ref().unwrap().as_str()).await?;
    if query_user_count != 0 {
        return Err(AppError::duplicate("user is already exist"));
    }
    Ok(1)
}

// 查询用户是否存在
pub async fn user_count(client: &Client, user_name: &str) -> Result<u64> {
    let filter = doc! {"user_name": user_name};
    let count = client
        .database(RUST_WEB_DB)
        .collection::<User>(USER_TABLE)
        .count_documents(filter, None)
        .await
        .unwrap();
    Ok(count)
}

// 查询用户
async fn find_user(client: &Client, user_name: &str) -> Result<User> {
    let user_info = client
        .database(RUST_WEB_DB)
        .collection::<User>(USER_TABLE)
        .find_one(doc! {"user_name": user_name,"is_del": 0}, None)
        .await
        .unwrap()
        .ok_or(AppError::notfound());
    user_info
}

// 查询用户
async fn find_user_by_id(client: &Client, user_id: &str) -> Result<User> {
    let user_info = client
        .database(RUST_WEB_DB)
        .collection::<User>(USER_TABLE)
        .find_one(doc! {"_id": user_id,"is_del": 0}, None)
        .await
        .unwrap()
        .ok_or(AppError::notfound());
    user_info
}

// 用户信息
pub async fn user_info(client: &Client, user_id: &str) -> Result<UserInfoDisPlay> {
    let user_info = find_user_by_id(client, user_id).await?;
    let user_display: UserInfoDisPlay = UserInfoDisPlay {
        user_id: user_info.user_id,
        user_name: user_info.user_name,
    };
    Ok(user_display)
}

// 用户注销
pub async fn logout(user_id: &str) -> Result<bool> {
    let rd_client = get_redis_client().unwrap();

    let mut con = rd_client.get_connection()?;

    let token_key = format!("token:{}", user_id);

    let _: () = redis::cmd("DEL").arg(token_key).query(&mut con)?;

    Ok(true)
}
