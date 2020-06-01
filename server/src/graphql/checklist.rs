use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};

#[derive(Debug)]
pub struct Context {}

impl juniper::Context for Context {}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A todo entry")]
pub struct Todo {
    pub id: String,
    pub name: String,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A todo entry")]
pub struct NewTodo {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn todo(_context: &Context, id: String) -> FieldResult<Todo> {
        Ok(Todo {
            id,
            name: "test".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn add_todo(_context: &Context, id: String, name: String) -> FieldResult<Todo> {
        Ok(Todo { id, name })
    }
}
