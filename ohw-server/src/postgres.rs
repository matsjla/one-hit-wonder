use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Database = Pool<Postgres>;

pub async fn create_connection_pool(database_url: &str) -> Result<Database, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await?;
    Ok(pool)
}
