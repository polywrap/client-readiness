import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient,
  Uri
} from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  const inputUri = Input.expectUri(input);
  const uri = Uri.from(
    inputUri.authority + "/" +
    Input.expectRootDir(
      inputUri.path,
      path.join(__dirname, "../../../../")
    )
  );

  console.log(`URI Authority: ${uri.authority}`);

  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving: ${input}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
