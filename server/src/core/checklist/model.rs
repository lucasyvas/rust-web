use super::super::common::model::Error as ModelError;
use anyhow::{Error, Result};
use sqlx::{Error::Database as DatabaseError, PgPool};
use std::sync::Arc;
use uuid::Uuid;

const INSERT_LIST: &str = "
  INSERT INTO lists (id, name)
  VALUES ($1, $2);
";

const INSERT_TODO: &str = "
  INSERT INTO todos (list_id, id, description, done)
  VALUES ($1, $2, $3, $4);
";

#[derive(Debug, sqlx::FromRow)]
pub struct TodoList {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub list_id: Uuid,
    pub id: Uuid,
    pub description: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct Model {
    pool: Arc<PgPool>,
}

impl Model {
    pub fn new(pool: Arc<PgPool>) -> Model {
        Model { pool }
    }

    pub async fn create_list(&self, id: &Option<&Uuid>, name: &str) -> Result<TodoList> {
        let id = match *id {
            Some(id) => id.to_owned(),
            None => Uuid::new_v4(),
        };

        let list = TodoList {
            id,
            name: name.to_string(),
        };

        let result = sqlx::query(INSERT_LIST)
            .bind(&list.id.to_hyphenated().to_string())
            .bind(&list.name)
            .execute(self.pool.as_ref())
            .await;

        let error = match result {
            Ok(_) => return Ok(list),
            Err(error) => error,
        };

        let database_error = match &error {
            DatabaseError(error) => error,
            _ => return Err(Error::new(ModelError::Sqlx(error))),
        };

        let error_code = match database_error.code() {
            None => return Err(Error::new(ModelError::Sqlx(error))),
            Some(code) => code,
        };

        let model_error = match error_code {
            "23505" => ModelError::Conflict(id),
            _ => ModelError::Sqlx(error),
        };

        Err(Error::new(model_error))
    }

    pub async fn create_todo(&self, list_id: &Uuid, description: &str) -> Result<Todo> {
        let todo = Todo {
            list_id: list_id.to_owned(),
            id: Uuid::new_v4(),
            description: description.to_string(),
            done: false,
        };

        sqlx::query(INSERT_TODO)
            .bind(&todo.list_id.to_hyphenated().to_string())
            .bind(&todo.id.to_hyphenated().to_string())
            .bind(&todo.description)
            .bind(&todo.done)
            .execute(self.pool.as_ref())
            .await?;

        Ok(todo)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::database;
    use super::*;
    use dotenv::dotenv;
    use pretty_assertions::assert_eq;
    use std::env;
    use std::sync::Arc;

    async fn setup() -> Result<Arc<PgPool>> {
        dotenv().ok();
        let pool = database::create_pool(&env::var("DATABASE_URL")?).await?;
        database::create_schema(&pool).await?;
        Ok(pool)
    }

    async fn create_model() -> Result<Model> {
        let pool = setup().await?;
        Ok(Model::new(pool.clone()))
    }

    #[tokio::test]
    async fn create_list() -> Result<()> {
        let model = create_model().await?;

        let list_id = Uuid::new_v4();
        let list_name = "new_list";
        let list = model.create_list(&Some(&list_id), &list_name).await?;

        assert_eq!(list.id, list_id);
        assert_eq!(list.name, list_name);

        Ok(())
    }

    #[tokio::test]
    async fn create_todo() -> Result<()> {
        let model = create_model().await?;

        let list_id = Uuid::new_v4();
        let todo_description = "new_todo";
        let todo = model.create_todo(&list_id, &todo_description).await?;

        assert_eq!(todo.list_id, list_id);
        assert_eq!(todo.description, todo_description);
        assert_eq!(todo.done, false);

        Ok(())
    }
}
