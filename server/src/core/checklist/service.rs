use super::model::{Model, Todo};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Service {
    model: Model,
}

impl Service {
    pub fn new(model: Model) -> Arc<Mutex<Service>> {
        Arc::new(Mutex::new(Service { model }))
    }

    pub async fn add_todo(&mut self, name: &str) -> Result<Todo> {
        Ok(self.model.create_todo(name).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::database;
    use super::*;
    use std::env;

    async fn setup() -> Result<Arc<database::Pool>> {
        let mut pool = database::create_pool(&env::var("DATABASE_URL")?).await?;
        database::create_schema(Arc::make_mut(&mut pool)).await?;
        Ok(pool)
    }

    async fn create_model() -> Result<Model> {
        let pool = setup().await?;
        Ok(Model::new(pool.clone()))
    }

    async fn create_service() -> Result<Arc<Mutex<Service>>> {
        Ok(Service::new(create_model().await?))
    }

    #[tokio::test]
    async fn add_todo() -> Result<()> {
        let service = create_service().await?;

        let todo_name = "new_todo";
        let todo = service.lock().await.add_todo(&todo_name).await?;

        assert_eq!(todo.name, todo_name);

        Ok(())
    }
}
