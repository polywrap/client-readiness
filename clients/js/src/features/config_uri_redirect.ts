import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    from: unknown;
    to: unknown;
  }>(input);
  const from = Input.expectUri(inputObj.from).uri;
  const to = Input.expectUri(inputObj.to).uri;

  console.log("Adding URI Redirect to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .setRedirect(from, to)
    .build();

  const client = new PolywrapClient(config);

  console.log("Resolving Redirect");

  const result = await client.tryResolveUri({ uri: from });

  if (result.ok && result.value.type === "uri") {
    console.log(`Received URI '${result.value.uri}'`);
    console.log("Success!");
  }
}
