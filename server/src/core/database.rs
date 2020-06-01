use anyhow::Result;
use std::sync::Arc;

const CREATE_LISTS_TABLE: &str = "
  CREATE TABLE IF NOT EXISTS lists (
    id CHAR(36) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
  );
";

const CREATE_TODOS_TABLE: &str = "
  CREATE TABLE IF NOT EXISTS todos (
    id CHAR(36) PRIMARY KEY NOT NULL,
    list_id CHAR(36) NOT NULL,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL
  );
";

pub type Pool = sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<Arc<Pool>> {
    Ok(Arc::new(
        Pool::builder().min_size(1).build(&database_url).await?,
    ))
}

pub async fn create_schema(pool: &Pool) -> Result<()> {
    sqlx::query(CREATE_LISTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_TODOS_TABLE).execute(pool).await?;
    Ok(())
}
