use anyhow::Result;
use std::sync::Arc;

pub type Pool = sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<Arc<Pool>> {
    Ok(Arc::new(
        Pool::builder().min_size(1).build(&database_url).await?,
    ))
}

pub async fn create_schema(pool: &mut Pool) -> Result<()> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS checklist (
          id CHAR(36) PRIMARY KEY NOT NULL,
          name TEXT NOT NULL
        )
        ",
    )
    .execute(pool)
    .await?;

    Ok(())
}
