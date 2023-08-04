import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient,
  ExtendableUriResolver,
  Uri,
} from "@polywrap/client-js";
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
  );

  const manifest = fs.readFileSync(path.join(wrapDir, "wrap.info"));
  const wasmModule = fs.readFileSync(path.join(wrapDir, "wrap.wasm"));

  const uri = Input.expectUri(inputObj.uri);
  const wrap = {
    package: WasmPackage.from(manifest, wasmModule),
    uri,
  };

  const config = new PolywrapClientConfigBuilder()
    .setPackage(wrap.uri.uri, wrap.package)
    .addInterfaceImplementation(
      ExtendableUriResolver.defaultExtInterfaceUris[0].uri,
      uri.uri
    )
    .build();

  const client = new PolywrapClient(config);

  console.log("Resolving URI wrap://expected-error/uri")

  const result = await client.tryResolveUri(Uri.from("expected-error/wrap"));
  if (!result.ok) {
    // @ts-ignore
    console.log("Received error: " + result.error?.reason);
  }
}
