mod checklist {
    tonic::include_proto!("checklist");
}

use super::service::Service;
use checklist::checklist_server::{Checklist, ChecklistServer};
use checklist::{AddTodoReply, AddTodoRequest};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Router {
    service: Arc<Service>,
}

impl Router {
    pub fn new(service: Arc<Service>) -> ChecklistServer<Router> {
        ChecklistServer::new(Router { service })
    }
}

#[tonic::async_trait]
impl Checklist for Router {
    async fn add_todo(&self, request: Request<AddTodoRequest>) -> Result<Response<AddTodoReply>, Status> {
        let todo = self.service.add_todo(request.into_inner().name.as_ref()).await;

        let todo = AddTodoReply {
            id: todo.id.to_hyphenated().to_string(),
            name: todo.name,
        };

        Ok(Response::new(todo))
    }
}
