use super::super::common;
use super::model::{Model, Todo, TodoList};
use anyhow::{Error, Result};
use common::model::Error as ModelError;
use common::service::Error as ServiceError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Service {
    model: Model,
}

impl Service {
    pub fn new(model: Model) -> Arc<Service> {
        Arc::new(Service { model })
    }

    pub async fn add_list(&self, id: &Option<&Uuid>, name: &str) -> Result<TodoList> {
        let result = self.model.create_list(id, name).await;

        match result {
            Ok(list) => return Ok(list),
            Err(err) => return Err(create_conflict_error(err)),
        };
    }

    pub async fn get_list(&self, id: &Uuid) -> Result<TodoList> {
        let result = self.model.get_list(id).await;

        match result {
            Ok(list) => return Ok(list),
            Err(err) => return Err(create_not_found_error(err)),
        };
    }

    pub async fn update_list(&self, id: &Uuid, name: &str) -> Result<TodoList> {
        let result = self.model.update_list(id, name).await;

        match result {
            Ok(list) => return Ok(list),
            Err(err) => return Err(create_not_found_error(err)),
        };
    }

    pub async fn remove_list(&self, id: &Uuid) -> Result<()> {
        let result = self.model.destroy_list(id).await;

        match result {
            Ok(list) => return Ok(list),
            Err(err) => return Err(create_not_found_error(err)),
        };
    }

    pub async fn add_todo(&self, list_id: &Uuid, description: &str) -> Result<Todo> {
        let result = self.model.create_todo(list_id, description).await;

        match result {
            Ok(todo) => return Ok(todo),
            Err(err) => return Err(create_validation_error(err)),
        };
    }
}

fn create_conflict_error(error: Error) -> Error {
    match error.downcast_ref::<ModelError>() {
        Some(ModelError::Conflict(id)) => return Error::new(ServiceError::Conflict(*id)),
        _ => return error,
    }
}

fn create_not_found_error(error: Error) -> Error {
    match error.downcast_ref::<ModelError>() {
        Some(ModelError::NotFound(id)) => return Error::new(ServiceError::NotFound(*id)),
        _ => return error,
    }
}

fn create_validation_error(error: Error) -> Error {
    match error.downcast_ref::<ModelError>() {
        Some(ModelError::Validation(msg)) => {
            return Error::new(ServiceError::Validation(msg.to_string()))
        }
        _ => return error,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::database;
    use super::*;
    use dotenv::dotenv;
    use pretty_assertions::assert_eq;
    use sqlx::PgPool;
    use std::env;

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

    async fn create_service() -> Result<Arc<Service>> {
        Ok(Service::new(create_model().await?))
    }

    #[tokio::test]
    async fn add_list() -> Result<()> {
        let service = create_service().await?;

        let list_id = Uuid::new_v4();
        let list_name = "new_list";
        let list = service.add_list(&Some(&list_id), &list_name).await?;

        assert_eq!(list.id, list_id);
        assert_eq!(list.name, list_name);

        Ok(())
    }

    #[tokio::test]
    async fn add_todo() -> Result<()> {
        let service = create_service().await?;

        let list_id = Uuid::new_v4();
        let todo_description = "new_todo";
        let todo = service.add_todo(&list_id, &todo_description).await?;

        assert_eq!(todo.list_id, list_id);
        assert_eq!(todo.description, todo_description);
        assert_eq!(todo.done, false);

        Ok(())
    }
}
