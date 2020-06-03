mod checklist {
    tonic::include_proto!("checklist");
}

use super::super::core::checklist::service::Service;
use checklist::checklist_server::{Checklist, ChecklistServer};

use checklist::{
    AddListRequest, AddTodoRequest, EmptyReply, GetListRequest, ListReply, RemoveListRequest,
    TodoReply, UpdateListRequest,
};

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

    async fn get_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        let id = request.into_inner().id;

        let id = match Uuid::parse_str(id.as_ref()) {
            Err(_) => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    format!("ID '{}' is invalid", id),
                ))
            }
            Ok(id) => id,
        };

        let list = self.service.get_list(&id).await;

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

    async fn update_list(
        &self,
        request: Request<UpdateListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        let UpdateListRequest { id, name } = request.into_inner();

        let id = match Uuid::parse_str(id.as_ref()) {
            Err(_) => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    format!("ID '{}' is invalid", id),
                ))
            }
            Ok(id) => id,
        };

        let list = self.service.update_list(&id, &name).await;

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

    async fn remove_list(
        &self,
        request: Request<RemoveListRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        let id = request.into_inner().id;

        let id = match Uuid::parse_str(id.as_ref()) {
            Err(_) => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    format!("ID '{}' is invalid", id),
                ))
            }
            Ok(id) => id,
        };

        if let Err(err) = self.service.remove_list(&id).await {
            return Err(Status::new(tonic::Code::Internal, err.to_string()));
        }

        Ok(Response::new(EmptyReply {}))
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
