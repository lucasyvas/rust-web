use super::model::{Model, Todo, TodoList};
use anyhow::Result;
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
        Ok(self.model.create_list(id, name).await?)
    }

    pub async fn add_todo(&self, list_id: &Uuid, description: &str) -> Result<Todo> {
        Ok(self.model.create_todo(list_id, description).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::database;
    use super::*;
    use dotenv::dotenv;
    use pretty_assertions::assert_eq;
    use std::env;

    async fn setup() -> Result<Arc<database::Pool>> {
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
