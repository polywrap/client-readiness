import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    interfaceUri: unknown;
    implementations: unknown;
  }>(input);
  const interfaceUri = Input.expectUri(inputObj.interfaceUri).uri;
  const implementations = Input.expectArray<string>(inputObj.implementations);

  console.log("Adding Interface Implementations to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .addInterfaceImplementations(
      interfaceUri,
      implementations
    )
    .build();

  const client = new PolywrapClient(config);

  console.log("Getting Implementations");

  const result = await client.getImplementations(interfaceUri);

  if (result.ok && result.value.length > 0) {
    console.log(`Found ${result.value.length} Implementations`);
    console.log("Success!");
  }
}
