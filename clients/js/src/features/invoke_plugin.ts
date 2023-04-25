import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "invoke_plugin test case input must be an object"
    );
  }

  const { uri, method, args } = input as any;

  if (typeof uri !== "string") {
    throw Error(
      "invoke_plugin input.uri must be a string"
    );
  }

  if (typeof method !== "string") {
    throw Error(
      "invoke_plugin input.method must be a string"
    );
  }

  if (typeof args !== "object") {
    throw Error("invoke_plugin input.args must be an object");
  }

  const pluginPackage = PluginPackage.from(() => ({
    add: (args: { a: number, b: number }) => {
      return args.a + args.b;
    }
  }));

  const config = new ClientConfigBuilder()
    .addPackage(uri, pluginPackage)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  const result = await client.invoke({
    uri,
    method,
    args
  });

  if (result.ok) {
    console.log(`Received ${result.value}`);
    console.log("Success!");
  }
}
