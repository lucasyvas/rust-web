import { config } from "dotenv";

config();

import { promisify } from "util";
import grpc from "grpc";
import { ChecklistClient } from "../proto/checklist_grpc_pb";
import { AddListRequest, AddTodoRequest, ListReply, TodoReply } from "../proto/checklist_pb";

async function addList(client: ChecklistClient, name: string) {
  const request = new AddListRequest();

  request.setName(name);

  const addList = promisify(client.addList.bind(client));
  const reply = <ListReply>await addList(request);

  return { id: reply.getId(), name: reply.getName() };
}

async function addTodo(client: ChecklistClient, listId: string, description: string) {
  const request = new AddTodoRequest();

  request.setListId(listId);
  request.setDescription(description);

  const addTodo = promisify(client.addTodo.bind(client));
  const reply = <TodoReply>await addTodo(request);

  return { listId: reply.getListId(), id: reply.getId(), description: reply.getDescription(), done: reply.getDone() };
}

async function main() {
  const checklistClient = new ChecklistClient(<string>process.env.SOCKET_ADDR, grpc.credentials.createInsecure());

  const list = await addList(checklistClient, "TODOs");
  const todo = await addTodo(checklistClient, list.id, "Get groceries");

  console.log(todo);
}

main();
