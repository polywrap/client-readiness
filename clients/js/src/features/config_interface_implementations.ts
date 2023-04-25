import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_interface_implementations test case input must be an object"
    );
  }

  const { interfaceUri, implementations } = input as any;

  if (typeof interfaceUri !== "string") {
    throw Error(
      "config_interface_implementations input.interfaceUri must be a string"
    );
  }

  if (!implementations || !Array.isArray(implementations)) {
    throw Error(
      "config_interface_implementations input.implementations must be an array"
    );
  }

  console.log("Adding Interface Implementations to ClientConfig");

  const config = new ClientConfigBuilder()
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
