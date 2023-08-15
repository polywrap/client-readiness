import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient,
  Uri,
} from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    directory: unknown;
    method: unknown;
    args: unknown;
    expectedError: unknown;
  }>(input);
  const wrapDir = Input.expectRootDir(
    inputObj.directory,
    path.join(__dirname, "../../../../")
  );
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(inputObj.args);

  const manifest = fs.readFileSync(path.join(wrapDir, "wrap.info"));
  const wasmModule = fs.readFileSync(path.join(wrapDir, "wrap.wasm"));

  const wrap = {
    package: WasmPackage.from(manifest, wasmModule),
    uri: Uri.from("wrap/invoke"),
  };

  const config = new PolywrapClientConfigBuilder()
    .setPackage(wrap.uri.uri, wrap.package)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking method ${method}`);

  const result = await client.invoke({
    uri: wrap.uri,
    method,
    args,
  });

  const expectedError = Input.expectString(inputObj.expectedError);

  if (!result.ok) {
    if (result.error?.toString().includes(expectedError)) {
      console.log("Expected error received");
    } else {
      console.log(
        `Expected error "${expectedError}", but received "${result.error}"`
      );
    }
  }
}
