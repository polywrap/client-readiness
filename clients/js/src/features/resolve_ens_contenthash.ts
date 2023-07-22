import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient
} from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  const uri = Input.expectUri(input);

  console.log(`URI Authority: ${uri.authority}`);

  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving: ${uri}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
