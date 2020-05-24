use super::model::{Model, Todo};
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct Service {
    model: Model,
}

impl Service {
    pub fn new(model: Model) -> Arc<Service> {
        Arc::new(Service { model })
    }

    pub async fn add_todo(&self, name: &str) -> Result<Todo> {
        Ok(self.model.create_todo(name).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::database;
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

    async fn create_service() -> Result<Arc<Service>> {
        Ok(Service::new(create_model().await?))
    }

    #[tokio::test]
    async fn add_todo() -> Result<()> {
        let service = create_service().await?;

        let todo_name = "new_todo";
        let todo = service.add_todo(&todo_name).await?;

        assert_eq!(todo.name, todo_name);
        teardown().await?;

        Ok(())
    }
}
