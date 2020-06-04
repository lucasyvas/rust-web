use super::super::common::model::Error as ModelError;
use super::super::database;
use anyhow::{Error, Result};
use database::ErrorCode as DatabaseErrorCode;
use sqlx::{postgres::PgQueryAs, Error as SqlxError, PgPool};
use std::sync::Arc;
use uuid::Uuid;

const INSERT_LIST: &str = "
  INSERT INTO lists (id, name)
  VALUES ($1, $2);
";

const SELECT_LIST: &str = "
  SELECT id, name FROM lists
  WHERE id = $1;
";

const UPDATE_LIST: &str = "
  UPDATE lists
  SET name = $2
  WHERE id = $1
  RETURNING id, name;
";

const DELETE_LIST: &str = "
  DELETE FROM lists
  WHERE id = $1
  RETURNING id;
";

const INSERT_TODO: &str = "
  INSERT INTO todos (list_id, id, description, done)
  VALUES ($1, $2, $3, $4);
";

#[derive(Debug)]
pub struct TodoList {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
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
            Err(err) => err,
        };

        let error_code = match extract_database_error_code(&error) {
            Err(_) => return Err(Error::new(error)),
            Ok(code) => code,
        };

        match error_code.as_ref() {
            DatabaseErrorCode::UniqueViolation => return Err(Error::new(ModelError::Conflict(id))),
            _ => return Err(Error::new(error)),
        };
    }

    pub async fn get_list(&self, id: &Uuid) -> Result<TodoList> {
        let result = sqlx::query_as::<_, (String, String)>(SELECT_LIST)
            .bind(id.to_hyphenated().to_string())
            .fetch_one(self.pool.as_ref())
            .await;

        let row = match result {
            Err(err) => return Err(create_not_found_error(err, id)),
            Ok(row) => row,
        };

        let list = create_list_from_row(row)?;

        Ok(list)
    }

    pub async fn update_list(&self, id: &Uuid, name: &str) -> Result<TodoList> {
        let result = sqlx::query_as::<_, (String, String)>(UPDATE_LIST)
            .bind(id.to_hyphenated().to_string())
            .bind(name)
            .fetch_one(self.pool.as_ref())
            .await;

        let row = match result {
            Err(err) => return Err(create_not_found_error(err, id)),
            Ok(row) => row,
        };

        let list = create_list_from_row(row)?;

        Ok(list)
    }

    pub async fn destroy_list(&self, id: &Uuid) -> Result<()> {
        let result = sqlx::query_as::<_, (String,)>(DELETE_LIST)
            .bind(&id.to_hyphenated().to_string())
            .fetch_one(self.pool.as_ref())
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(create_not_found_error(err, id)),
        };
    }

    pub async fn create_todo(&self, list_id: &Uuid, description: &str) -> Result<Todo> {
        let todo = Todo {
            list_id: list_id.to_owned(),
            id: Uuid::new_v4(),
            description: description.to_string(),
            done: false,
        };

        let result = sqlx::query(INSERT_TODO)
            .bind(&todo.list_id.to_hyphenated().to_string())
            .bind(&todo.id.to_hyphenated().to_string())
            .bind(&todo.description)
            .bind(&todo.done)
            .execute(self.pool.as_ref())
            .await;

        let error = match result {
            Ok(_) => return Ok(todo),
            Err(err) => err,
        };

        let error_code = match extract_database_error_code(&error) {
            Err(_) => return Err(Error::new(error)),
            Ok(code) => code,
        };

        match error_code.as_ref() {
            DatabaseErrorCode::ForeignKeyViolation => {
                return Err(Error::new(ModelError::Validation(format!(
                    "list ID '{}' not in collection",
                    list_id
                ))))
            }
            _ => return Err(Error::new(error)),
        };
    }
}

fn create_list_from_row(row: (String, String)) -> Result<TodoList> {
    let list = TodoList {
        id: Uuid::parse_str(row.0.as_ref())?,
        name: row.1,
    };

    Ok(list)
}

fn create_not_found_error(error: SqlxError, id: &Uuid) -> Error {
    match error {
        SqlxError::RowNotFound => return Error::new(ModelError::NotFound(id.to_owned())),
        _ => return Error::new(error),
    }
}

fn extract_database_error_code(error: &SqlxError) -> std::result::Result<String, &SqlxError> {
    let database_error = match error {
        SqlxError::Database(err) => err,
        _ => return Err(error),
    };

    match database_error.code() {
        None => return Err(error),
        Some(code) => return Ok(code.to_string()),
    };
}

#[cfg(test)]
mod tests {
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
