#![allow(non_snake_case)]

use crate::common::response::ResVO;
use mongodb::Client;

/**
 * @desc AppState  App 状态
 */
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Client,
}

pub fn CUSTOM_AUTHORIZATION_ERROR() -> ResVO<()> {
    let AUTHORIZATION_ERROR: ResVO<()> =
        ResVO::from_error(Some(403), "Authorization Error".to_string(), None);
    AUTHORIZATION_ERROR
}
