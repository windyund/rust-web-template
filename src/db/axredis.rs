use crate::common::errors::AppError;
use once_cell::sync::OnceCell;
use redis::RedisError;
use redis::{Client, RedisResult};

static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

// 建立Redis 连接
pub async fn init_redis_pool() -> Result<(), AppError> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL is not set.");
    let client = match Client::open(redis_url) {
        Ok(client) => {
            tracing::debug!("✅ Connection to the redis is successful!");
            client
        }
        Err(err) => {
            tracing::debug!("🔥 Failed to connect to the redis: {:?}", err);
            std::process::exit(1);
        }
    };
    assert!(REDIS_CLIENT.set(client).is_ok());
    Ok(())
}

pub fn get_redis_client() -> Option<&'static Client> {
    REDIS_CLIENT.get()
}
