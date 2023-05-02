import { Input } from "../input";

import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PluginPackage } from "@polywrap/plugin-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    rootWrap: unknown;
    subWrapUri: unknown;
    method: unknown;
    args: unknown;
  }>(input);
  const subWrapUri = Input.expectUri(inputObj.subWrapUri);
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  const rootWrapObj = Input.expectObject<{
    directory: unknown;
    uri: unknown;
  }>(inputObj.rootWrap);

  const rootWrapDirectory = Input.expectRootDir(
    rootWrapObj.directory,
    path.join(__dirname, "../../../../")
  );

  const rootWrapUri = Input.expectUri(rootWrapObj.uri);
  const rootWrapPackage = WasmPackage.from(
    fs.readFileSync(path.join(rootWrapDirectory, "wrap.info")),
    fs.readFileSync(path.join(rootWrapDirectory, "wrap.wasm"))
  );

  const subWrapPackage = PluginPackage.from(() => ({
    add: (args: { a: number, b: number }) => {
      return args.a + args.b;
    }
  }));

  const config = new ClientConfigBuilder()
    .addPackages({
      [rootWrapUri.uri]: rootWrapPackage,
      [subWrapUri.uri]: subWrapPackage
    })
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  const result = await client.invoke({
    uri: rootWrapUri,
    method,
    args
  });

  if (result.ok) {
    console.log(`Received: ${result.value}`);
    console.log("Success!");
  }
}
