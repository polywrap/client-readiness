import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

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

  console.log(`Reading wrap.info & wrap.wasm from ${inputObj.directory}`);

  const manifest = fs.readFileSync(
    path.join(wrapDir, "wrap.info")
  );
  const wasmModule = fs.readFileSync(
    path.join(wrapDir, "wrap.wasm")
  );

  console.log("Creating WrapPackage from raw wrap.info & wrap.wasm bytes");

  const wrapPackage = WasmPackage.from(
    manifest,
    wasmModule
  );

  console.log("Adding WrapPackage to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .setPackage("embed/foo", wrapPackage)
    .build();

  const client = new PolywrapClient(config);

  console.log("Invoking WrapPackage");

  const result = await client.invoke({
    uri: "embed/foo",
    method,
    args
  });

  if (result.ok) {
    console.log("Success!");
  }
}
