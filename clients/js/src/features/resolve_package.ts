import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    directory: unknown;
    uri: unknown;
  }>(input);
  const wrapDir = Input.expectRootDir(
    inputObj.directory,
    path.join(__dirname, "../../../../")
  )
  const uri = Input.expectUri(inputObj.uri).uri;

  const manifest = fs.readFileSync(
    path.join(wrapDir, "wrap.info")
  );
  const wasmModule = fs.readFileSync(
    path.join(wrapDir, "wrap.wasm")
  );
  const wrapPackage = WasmPackage.from(
    manifest,
    wasmModule
  );

  const config = new PolywrapClientConfigBuilder()
    .setPackage(uri, wrapPackage)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving URI: ${uri}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
