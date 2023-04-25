import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_embed_wrap_package test case input must be a string, and start with $ROOT/"
    );
  }

  const { directory, method, args } = input as any;

  if (
    typeof directory !== "string" ||
    !directory.includes("$ROOT/")
  ) {
    throw Error(
      "config_embed_wrap_package input.directory must be a string, and start with $ROOT/"
    );
  }

  if (typeof method !== "string") {
    throw Error("config_embed_wrap_package input.method must be a string");
  }

  if (typeof args !== "object") {
    throw Error("config_embed_wrap_package input.args must be an object");
  }

  const wrapDir = directory.replace(
    "$ROOT/",
    path.join(__dirname, "../../../../")
  );

  console.log(`Reading wrap.info & wrap.wasm from ${directory}`);

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

  const config = new ClientConfigBuilder()
    .addPackage(
      "embed/foo",
      wrapPackage
    )
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
