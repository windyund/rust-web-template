use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use once_cell::sync::OnceCell;
use std::env;

static MYSQL_POOL: OnceCell<Client> = OnceCell::new();

// 建立 Mongo 连接
pub async fn init_db_pool() -> Result<Client, Error> {
    let key = "MONGO_URL";
    let url = env::var(key).expect("MONGO_URL is not set.");

    let mut client_options = ClientOptions::parse(&url).await.unwrap();

    // Manually set an option.
    client_options.app_name = Some("Rust Web App".to_string());
    client_options.max_connecting = Some(10);

    // Get a handle to the deployment.
    let client = match Client::with_options(client_options) {
        Ok(pool) => {
            tracing::debug!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::debug!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    Ok(client)
}

// 获取数据库
pub fn get_pool() -> Option<&'static Client> {
    MYSQL_POOL.get()
}
