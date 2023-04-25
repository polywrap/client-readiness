import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "invoke_wrap_wasm_v0_1 test case input must be an object"
    );
  }

  const { directory, method, args } = input as any;

  if (
    typeof directory !== "string" ||
    !directory.includes("$ROOT/")
  ) {
    throw Error(
      "invoke_wrap_wasm_v0_1 input.directory must be a string, and start with $ROOT/"
    );
  }

  if (typeof method !== "string") {
    throw Error("invoke_wrap_wasm_v0_1 input.method must be a string");
  }

  if (typeof args !== "object") {
    throw Error("invoke_wrap_wasm_v0_1 input.args must be an object");
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
    console.log(`Received ${result.value}`);
    console.log("Success!");
  }
}
