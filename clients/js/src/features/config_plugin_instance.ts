import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginWrapper, PluginModule } from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    uri: unknown;
    method: unknown;
  }>(input);
  const uri = Input.expectUri(inputObj.uri).uri;
  const method = Input.expectString(inputObj.method);

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

  const config = new PolywrapClientConfigBuilder()
    .setWrapper(uri, pluginWrapper)
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
