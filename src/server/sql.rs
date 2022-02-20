use std::sync::Mutex;

use sqlx::mysql::MySqlPool as Pool;
use static_init::dynamic;

#[dynamic]
pub static sql_pool: Mutex<Option<Pool>> = Mutex::new(None);