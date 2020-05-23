#[cfg(not(test))]
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct Model {
    #[cfg(not(test))]
    pool: Arc<SqlitePool>,
}

impl Model {
    pub fn new(#[cfg(not(test))] pool: Arc<SqlitePool>) -> Arc<Model> {
        #[rustfmt::skip]
        return Arc::new(Model { #[cfg(not(test))]pool });
    }

    pub async fn create_todo(&self, name: &str) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_model() -> Arc<Model> {
        Model::new()
    }

    #[tokio::test]
    async fn create_todo() {
        let model = create_model();

        let todo_name = "new_todo";
        let todo = model.create_todo(&todo_name).await;

        assert_eq!(todo.name, todo_name);
    }
}
