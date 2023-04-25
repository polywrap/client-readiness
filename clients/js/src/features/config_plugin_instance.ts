import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginWrapper, PluginModule } from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_plugin_instance test case input must be an object"
    );
  }

  const { uri, method } = input as any;

  if (typeof uri !== "string") {
    throw Error(
      "config_plugin_instance input.uri must be a string"
    );
  }

  if (typeof method !== "string") {
    throw Error(
      "config_plugin_instance input.method must be a string"
    );
  }

  console.log("Creating Plugin Instance");

  class Plugin extends PluginModule<{}, {}> {
    counter: number = 0;

    increment() {
      ++this.counter;
    }
  }

  const plugin = new Plugin({});

  console.log("Adding Plugin Instance to ClientConfig");

  const pluginWrapper = new PluginWrapper(
    { version: "0.1", type: "plugin", name: "counter", abi: {} },
    plugin
  );

  const config = new ClientConfigBuilder()
    .addWrapper(uri, pluginWrapper)
    .build();

  const client = new PolywrapClient(config);

  for (let i = 0; i < 2; ++i) {
    console.log("Invoking Plugin Instance");

    const result = await client.invoke({
      uri,
      method
    });

    if (result.ok) {
      console.log(`counter = ${plugin.counter}`);
    }
  }

  console.log("Success!");
}
