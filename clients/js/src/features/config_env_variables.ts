import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    uri: unknown;
    env: unknown;
  }>(input);
  const uri = Input.expectUri(inputObj.uri).uri;
  const env = Input.expectObject<Record<string, unknown>>(
    inputObj.env
  );

  console.log("Adding Env to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .addEnv(uri, env)
    .build();

  const client = new PolywrapClient(config);

  console.log("Fetching Env");

  const result = client.getEnvByUri(uri);

  if (result) {
    for (const key of Object.keys(result)) {
      console.log(`env.${key} = ${result[key]}`);
    }
    console.log("Success!");
  }
}
