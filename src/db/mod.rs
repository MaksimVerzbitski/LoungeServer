use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use std::env;

pub async fn create_pool() -> Pool {
    dotenv().ok();
    
    let mut cfg = Config::new();
    cfg.host = Some(env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"));
    cfg.dbname = Some(env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"));
    cfg.user = Some(env::var("DATABASE_USER").expect("DATABASE_USER must be set"));
    cfg.password = Some(env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"));
    cfg.port = Some(env::var("DATABASE_PORT").expect("DATABASE_PORT must be set").parse().expect("DATABASE_PORT must be a number"));
    
    match cfg.create_pool(NoTls) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            std::process::exit(1);
        }
    }
}



/* CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT NOT NULL,
    username VARCHAR(255) NOT NULL,
    tokens INT NOT NULL,
    referals INT NOT NULL,
    friends BIGINT NOT NULL,
    active_chat INT NOT NULL
); */