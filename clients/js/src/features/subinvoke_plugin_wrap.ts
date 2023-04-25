import { ClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PluginPackage } from "@polywrap/plugin-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "subinvoke_plugin_wrap test case input must be an object"
    );
  }

  const { directory, method, args } = input as any;

  if (
    typeof directory !== "string" ||
    !directory.includes("$ROOT/")
  ) {
    throw Error(
      "subinvoke_plugin_wrap input.directory must be a string, and start with $ROOT/"
    );
  }

  if (typeof method !== "string") {
    throw Error("subinvoke_plugin_wrap input.method must be a string");
  }

  if (typeof args !== "object") {
    throw Error("subinvoke_plugin_wrap input.args must be an object");
  }

  const wrapDir = directory.replace(
    "$ROOT/",
    path.join(__dirname, "../../../../")
  );

  const manifest = fs.readFileSync(
    path.join(wrapDir, "wrap.info")
  );
  const wasmModule = fs.readFileSync(
    path.join(wrapDir, "wrap.wasm")
  );

  const wrap = {
    package: WasmPackage.from(manifest, wasmModule),
    uri: Uri.from("embed/foo")
  };

  const plugin = {
    package: PluginPackage.from(() => ({
      performSubinvoke: async (_, client) => {
        console.log(`Subinvoking ${method}`);
        const res = await client.invoke({
          uri: wrap.uri,
          method,
          args
        });
        if (res.ok) {
          console.log(`Received ${res.value}`);
          return true;
        } else {
          return false;
        }
      }
    })),
    uri: Uri.from("plugin/bar")
  }; 

  const config = new ClientConfigBuilder()
    .addPackages({
      [wrap.uri.uri]: wrap.package,
      [plugin.uri.uri]: plugin.package
    })
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking Plugin`);

  const result = await client.invoke({
    uri: plugin.uri,
    method: "performSubinvoke"
  });

  if (result.ok && result.value) {
    console.log("Success!");
  }
}
