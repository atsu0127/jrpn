use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use anyhow::{Context, Result};

pub fn establish_connection() -> Result<MysqlConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(
        MysqlConnection::establish(&database_url)
        .with_context(|| {
            format!("Error connecting to {}", database_url)
        })?
    )
}