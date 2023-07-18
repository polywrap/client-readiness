import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    uri: unknown;
    method: unknown;
    args: unknown;
  }>(input);
  const uri = Input.expectUri(inputObj.uri).uri;
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  console.log("Creating PluginPackage");

  const pluginPackage = PluginPackage.from(() => ({
    add: (args: { a: number, b: number }) => {
      return args.a + args.b;
    }
  }));

  console.log("Adding PluginPackage to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .addPackage(uri, pluginPackage)
    .build();

  const client = new PolywrapClient(config);

  console.log("Invoking PluginPackage");

  const result = await client.invoke({
    uri,
    method,
    args
  });

  if (result.ok) {
    console.log("Success!");
  }
}
