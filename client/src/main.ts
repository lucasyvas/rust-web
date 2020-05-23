import { config } from "dotenv";

config();

import { promisify } from "util";
import grpc from "grpc";
import { ChecklistClient } from "../proto/checklist_grpc_pb";
import { AddTodoRequest, AddTodoReply } from "../proto/checklist_pb";

async function main() {
  const checklist = new ChecklistClient(<string>process.env.SOCKET_ADDR, grpc.credentials.createInsecure());

  const request = new AddTodoRequest();
  request.setName("wat");

  const addTodo = promisify(checklist.addTodo.bind(checklist));

  try {
    const reply = <AddTodoReply>await addTodo(request);
    console.log({ id: reply.getId(), name: reply.getName() });
  } catch (error) {
    console.error(error);
  }
}

main();
