import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_plugin_package test case input must be an object"
    );
  }

  const { uri, method, args } = input as any;

  if (typeof uri !== "string") {
    throw Error(
      "config_plugin_package input.uri must be a string"
    );
  }

  if (typeof method !== "string") {
    throw Error(
      "config_plugin_package input.method must be a string"
    );
  }

  if (typeof args !== "object") {
    throw Error("config_plugin_package input.args must be an object");
  }

  console.log("Creating PluginPackage");

  const pluginPackage = PluginPackage.from(() => ({
    add: (args: { a: number, b: number }) => {
      return args.a + args.b;
    }
  }));

  console.log("Adding PluginPackage to ClientConfig");

  const config = new ClientConfigBuilder()
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
