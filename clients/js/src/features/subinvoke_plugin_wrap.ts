import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PluginPackage } from "@polywrap/plugin-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    directory: unknown;
    method: unknown;
    args: unknown;
  }>(input);
  const wrapDir = Input.expectRootDir(
    inputObj.directory,
    path.join(__dirname, "../../../../")
  );
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
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
          console.log(`Received: ${res.value}`);
          return true;
        } else {
          return false;
        }
      }
    })),
    uri: Uri.from("plugin/bar")
  }; 

  const config = new PolywrapClientConfigBuilder()
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
