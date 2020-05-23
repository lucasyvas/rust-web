use super::model::{Model, Todo};
use std::sync::Arc;

#[derive(Debug)]
pub struct Service {
    model: Arc<Model>,
}

impl Service {
    pub fn new(model: Arc<Model>) -> Arc<Service> {
        Arc::new(Service { model })
    }

    pub async fn add_todo(&self, name: &str) -> Todo {
        self.model.create_todo(name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_service() -> Arc<Service> {
        Service::new(Model::new())
    }

    #[tokio::test]
    async fn add_todo() {
        let service = create_service();

        let todo_name = "new_todo";
        let todo = service.add_todo(&todo_name).await;

        assert_eq!(todo.name, todo_name);
    }
}
