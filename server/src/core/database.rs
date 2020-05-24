use anyhow::Result;
use std::sync::Arc;
#[cfg(test)]
use tokio::fs;

#[cfg(not(test))]
pub type Pool = sqlx::PgPool;
#[cfg(test)]
pub type Pool = sqlx::SqlitePool;

#[cfg(test)]
const TEST_DATABASE_FILENAME: &str = "test.db";

pub async fn create_pool(#[cfg(not(test))] database_url: String) -> Result<Arc<Pool>> {
    #[cfg(test)]
    let database_url = format!("sqlite:{}", TEST_DATABASE_FILENAME);

    Ok(Arc::new(
        Pool::builder().min_size(1).build(&database_url).await?,
    ))
}

pub async fn create_schema(pool: &Pool) -> Result<()> {
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

#[cfg(test)]
pub async fn destroy() -> Result<(), std::io::Error> {
    let err = match fs::remove_file(TEST_DATABASE_FILENAME).await {
        Ok(_) => return Ok(()),
        Err(err) => err,
    };

    if err.kind() != std::io::ErrorKind::NotFound {
        return Err(err);
    }

    Ok(())
}
