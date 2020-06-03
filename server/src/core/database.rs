use anyhow::Result;
use sqlx::PgPool;
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
    list_id CHAR(36) NOT NULL REFERENCES lists ON DELETE CASCADE,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL,
    FOREIGN KEY (list_id) REFERENCES lists (id)
  );
";

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ErrorCode {
    pub const ForeignKeyViolation: &str = "23503";
    pub const UniqueViolation: &str = "23505";
}

pub async fn create_pool(database_url: &str) -> Result<Arc<PgPool>> {
    Ok(Arc::new(
        PgPool::builder().min_size(1).build(&database_url).await?,
    ))
}

pub async fn create_schema(pool: &PgPool) -> Result<()> {
    sqlx::query(CREATE_LISTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_TODOS_TABLE).execute(pool).await?;
    Ok(())
}
