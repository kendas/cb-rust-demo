use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::configuration;

pub async fn get_db_pool() -> Pool<Postgres> {
    let mut config = configuration::get_configuration().unwrap();
    config.database.name = Uuid::new_v4().to_string();
    let config = config;
    let mut connection = PgConnection::connect(&config.database.connection_string_without_db())
        .await
        .unwrap();
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database.name))
        .await
        .unwrap();
    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    pool
}

#[cfg(test)]
pub mod internal {
    use sqlx::{pool::PoolConnection, Postgres};

    pub(crate) async fn get_db_connection() -> PoolConnection<Postgres> {
        super::get_db_pool().await.acquire().await.unwrap()
    }
}
