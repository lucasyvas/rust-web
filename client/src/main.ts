import { config } from "dotenv";

config();

import { promisify } from "util";
import grpc from "grpc";
import { ChecklistClient } from "../proto/checklist_grpc_pb";

import {
  AddListRequest,
  AddTodoRequest,
  GetListRequest,
  UpdateListRequest,
  RemoveListRequest,
  ListReply,
  TodoReply,
} from "../proto/checklist_pb";

async function addList(client: ChecklistClient, name: string) {
  const request = new AddListRequest();

  request.setName(name);

  const addList = promisify(client.addList.bind(client));
  const reply = <ListReply>await addList(request);

  return { id: reply.getId(), name: reply.getName() };
}

async function getList(client: ChecklistClient, id: string) {
  const request = new GetListRequest();

  request.setId(id);

  const getList = promisify(client.getList.bind(client));
  const reply = <ListReply>await getList(request);

  return { id: reply.getId(), name: reply.getName() };
}

async function updateList(client: ChecklistClient, id: string, name: string) {
  const request = new UpdateListRequest();

  request.setId(id);
  request.setName(name);

  const updateList = promisify(client.updateList.bind(client));
  const reply = <ListReply>await updateList(request);

  return { id: reply.getId(), name: reply.getName() };
}

async function removeList(client: ChecklistClient, id: string) {
  const request = new RemoveListRequest();

  request.setId(id);

  const removeList = promisify(client.removeList.bind(client));
  await removeList(request);
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

  // "79e1ad60-a0be-43c3-995b-2451f5b83ed7"

  const list = await addList(checklistClient, "TODOs");
  await getList(checklistClient, list.id);
  await updateList(checklistClient, list.id, "jabroni");
  await addTodo(checklistClient, list.id, "Get groceries");
  await removeList(checklistClient, list.id);
}

main();
