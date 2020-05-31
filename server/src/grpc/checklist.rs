mod checklist {
    tonic::include_proto!("checklist");
}

use super::super::core::checklist::service::Service;
use checklist::checklist_server::{Checklist, ChecklistServer};
use checklist::{AddTodoReply, AddTodoRequest};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Controller {
    service: Arc<Mutex<Service>>,
}

impl Controller {
    pub fn new(service: Arc<Mutex<Service>>) -> ChecklistServer<Controller> {
        ChecklistServer::new(Controller { service })
    }
}

#[tonic::async_trait]
impl Checklist for Controller {
    async fn add_todo(
        &self,
        request: Request<AddTodoRequest>,
    ) -> Result<Response<AddTodoReply>, Status> {
        let todo = self
            .service
            .lock()
            .await
            .add_todo(request.into_inner().name.as_ref())
            .await;

        let todo = match todo {
            Err(_) => return Err(Status::new(tonic::Code::Internal, "Internal Error")),
            Ok(todo) => AddTodoReply {
                id: todo.id.to_hyphenated().to_string(),
                name: todo.name,
            },
        };

        Ok(Response::new(todo))
    }
}
