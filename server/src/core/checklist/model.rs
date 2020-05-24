use super::super::database;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct Model {
    pool: Arc<database::Pool>,
}

impl Model {
    pub fn new(pool: Arc<database::Pool>) -> Model {
        Model { pool }
    }

    pub async fn create_todo(&self, name: &str) -> Result<Todo> {
        let todo = Todo {
            id: Uuid::new_v4(),
            name: name.to_string(),
        };

        sqlx::query(
            "
              INSERT INTO checklist (id, name)
              VALUES ($1, $2)
            ",
        )
        .bind(&todo.id.to_hyphenated().to_string())
        .bind(&todo.name)
        .execute(self.pool.as_ref())
        .await?;

        Ok(todo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Result<Arc<database::Pool>> {
        teardown().await?;
        let pool = database::create_pool().await?;
        database::create_schema(pool.as_ref()).await?;
        Ok(pool)
    }

    async fn teardown() -> Result<()> {
        database::destroy().await?;
        Ok(())
    }

    async fn create_model() -> Result<Model> {
        let pool = setup().await?;
        Ok(Model::new(pool))
    }

    #[tokio::test]
    async fn create_todo() -> Result<()> {
        let model = create_model().await?;

        let todo_name = "new_todo";
        let todo = model.create_todo(&todo_name).await?;

        assert_eq!(todo.name, todo_name);
        teardown().await?;

        Ok(())
    }
}
