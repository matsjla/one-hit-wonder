use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Database = Pool<Postgres>;

#[cfg(test)]
pub async fn create_testcontainers_pool<'a>(
    container: &'a testcontainers::Container<'a, PostgresImage>,
) -> Result<Database, sqlx::Error> {
    let host_port = container.get_host_port_ipv4(5432);
    let host = format!("postgres://postgres@127.0.0.1:{}/postgres", host_port);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&host)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub async fn create_connection_pool(database_url: &str) -> Result<Database, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await?;
    Ok(pool)
}
