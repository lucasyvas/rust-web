import dotenv from "dotenv";

dotenv.config();

import { promisify } from "util";
import grpc from "grpc";
import { GreeterClient } from "../proto/helloworld_grpc_pb";
import { HelloRequest, HelloReply } from "../proto/helloworld_pb";

async function main() {
  const greeter = new GreeterClient(<string>process.env.SOCKET_ADDR, grpc.credentials.createInsecure());

  const request = new HelloRequest();
  request.setName("wat");

  const sayHello = promisify(greeter.sayHello.bind(greeter));

  try {
    const reply = <HelloReply>await sayHello(request);
    console.log("Greeting:", reply.getMessage());
  } catch (error) {
    console.error(error);
  }
}

main();
