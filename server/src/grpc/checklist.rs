mod checklist {
    tonic::include_proto!("checklist");
}

use super::super::core::checklist::service::Service;
use super::super::core::common::service::Error as ServiceError;
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

        let result = self.service.add_list(&None, &name).await;

        match result {
            Ok(list) => {
                return Ok(Response::new(ListReply {
                    id: list.id.to_hyphenated().to_string(),
                    name: list.name,
                }));
            }
            Err(err) => {
                log::error!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, "Unknown Error"));
            }
        };
    }

    async fn get_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        let id = convert_id(request.into_inner().id.as_ref())?;

        let result = self.service.get_list(&id).await;

        let error = match result {
            Ok(list) => {
                return Ok(Response::new(ListReply {
                    id: list.id.to_hyphenated().to_string(),
                    name: list.name,
                }))
            }
            Err(err) => err,
        };

        let status = match error.downcast_ref::<ServiceError>() {
            Some(ServiceError::NotFound(_)) => {
                Status::new(tonic::Code::NotFound, error.to_string())
            }
            _ => {
                log::error!("{:?}", error);
                Status::new(tonic::Code::Internal, "Unknown Error")
            }
        };

        Err(status)
    }

    async fn update_list(
        &self,
        request: Request<UpdateListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        let UpdateListRequest { id, name } = request.into_inner();
        let id = convert_id(id.as_ref())?;

        let result = self.service.update_list(&id, &name).await;

        let error = match result {
            Ok(list) => {
                return Ok(Response::new(ListReply {
                    id: list.id.to_hyphenated().to_string(),
                    name: list.name,
                }))
            }
            Err(err) => err,
        };

        let status = match error.downcast_ref::<ServiceError>() {
            Some(ServiceError::NotFound(_)) => {
                Status::new(tonic::Code::NotFound, error.to_string())
            }
            _ => {
                log::error!("{:?}", error);
                Status::new(tonic::Code::Internal, "Unknown Error")
            }
        };

        Err(status)
    }

    async fn remove_list(
        &self,
        request: Request<RemoveListRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        let id = convert_id(request.into_inner().id.as_ref())?;

        let result = self.service.remove_list(&id).await;

        let error = match result {
            Ok(_) => return Ok(Response::new(EmptyReply {})),
            Err(err) => err,
        };

        let status = match error.downcast_ref::<ServiceError>() {
            Some(ServiceError::NotFound(_)) => {
                Status::new(tonic::Code::NotFound, error.to_string())
            }
            _ => {
                log::error!("{:?}", error);
                Status::new(tonic::Code::Internal, "Unknown Error")
            }
        };

        Err(status)
    }

    async fn add_todo(
        &self,
        request: Request<AddTodoRequest>,
    ) -> Result<Response<TodoReply>, Status> {
        let AddTodoRequest {
            list_id,
            description,
        } = request.into_inner();

        let list_id = convert_id(list_id.as_ref())?;

        let result = self.service.add_todo(&list_id, &description).await;

        let error = match result {
            Ok(todo) => {
                return Ok(Response::new(TodoReply {
                    list_id: todo.list_id.to_hyphenated().to_string(),
                    id: todo.id.to_hyphenated().to_string(),
                    description: todo.description,
                    done: todo.done,
                }))
            }
            Err(err) => err,
        };

        let status = match error.downcast_ref::<ServiceError>() {
            Some(ServiceError::Validation(_)) => {
                Status::new(tonic::Code::InvalidArgument, error.to_string())
            }
            _ => {
                log::error!("{:?}", error);
                Status::new(tonic::Code::Internal, "Unknown Error")
            }
        };

        Err(status)
    }
}

fn convert_id(id: &str) -> Result<Uuid, Status> {
    match Uuid::parse_str(id) {
        Err(_) => {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                format!("'{}' is not a valid v4 UUID", id),
            ))
        }
        Ok(id) => return Ok(id),
    };
}
