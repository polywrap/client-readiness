import { ClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import { WasmWrapper } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "resolve_package test case input must be an object"
    );
  }

  const { directory, uri } = input as any;

  if (
    typeof directory !== "string" ||
    !directory.includes("$ROOT/")
  ) {
    throw Error(
      "resolve_package input.directory must be a string, and start with $ROOT/"
    );
  }

  if (typeof uri !== "string" || !Uri.isValidUri(uri)) {
    throw Error("resolve_package input.method must be a valid URI");
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
  const wrapInstance = await WasmWrapper.from(
    manifest,
    wasmModule
  );

  const config = new ClientConfigBuilder()
    .addWrapper(uri, wrapInstance)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving URI: ${uri}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
