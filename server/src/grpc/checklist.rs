mod checklist {
    tonic::include_proto!("checklist");
}

use super::super::core::checklist::service::Service;
use checklist::checklist_server::{Checklist, ChecklistServer};
use checklist::{AddListRequest, AddTodoRequest, ListReply, TodoReply};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug)]
pub struct Controller {
    service: Arc<Service>,
}

impl Controller {
    pub fn new(service: Arc<Service>) -> ChecklistServer<Controller> {
        ChecklistServer::new(Controller { service })
    }
}

#[tonic::async_trait]
impl Checklist for Controller {
    async fn add_list(
        &self,
        request: Request<AddListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        let AddListRequest { name } = request.into_inner();

        let list = self.service.add_list(&None, &name).await;

        let list = match list {
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, err.to_string()));
            }
            Ok(list) => ListReply {
                id: list.id.to_hyphenated().to_string(),
                name: list.name,
            },
        };

        Ok(Response::new(list))
    }

    async fn add_todo(
        &self,
        request: Request<AddTodoRequest>,
    ) -> Result<Response<TodoReply>, Status> {
        let request = request.into_inner();

        let list_id = match Uuid::parse_str(&request.list_id) {
            Ok(list_id) => list_id,
            Err(_) => Uuid::new_v4(),
        };

        let todo = self.service.add_todo(&list_id, &request.description).await;

        let todo = match todo {
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, err.to_string()));
            }
            Ok(todo) => TodoReply {
                list_id: todo.list_id.to_hyphenated().to_string(),
                id: todo.id.to_hyphenated().to_string(),
                description: todo.description,
                done: todo.done,
            },
        };

        Ok(Response::new(todo))
    }
}
