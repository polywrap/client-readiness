import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_env_variables test case input must be an object"
    );
  }

  const { uri, env } = input as any;

  if (typeof uri !== "string") {
    throw Error(
      "config_env_variables input.uri must be a string"
    );
  }

  if (!env || typeof env !== "object") {
    throw Error(
      "config_env_variables input.env must be an object"
    );
  }

  console.log("Adding Env to ClientConfig");

  const config = new ClientConfigBuilder()
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
