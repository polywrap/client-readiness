import { Input } from "../input";

import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
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

  const config = new ClientConfigBuilder()
    .addPackage("embed/foo", wrapPackage)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  const result = await client.invoke({
    uri: "embed/foo",
    method,
    args
  });

  if (result.ok) {
    console.log(`Received: ${result.value}`);
    console.log("Success!");
  }
}
