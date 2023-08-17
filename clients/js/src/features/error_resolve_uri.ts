import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient,
  ExtendableUriResolver,
} from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    resolver?: {
      directory: unknown;
      uri: unknown;
    };
    uri: unknown;
    expectedError: unknown;
  }>(input);

  const resolver = inputObj.resolver;
  const config = new PolywrapClientConfigBuilder();

  if (resolver) {
    const wrapDir = Input.expectRootDir(
      resolver.directory,
      path.join(__dirname, "../../../../")
    );

    const manifest = fs.readFileSync(path.join(wrapDir, "wrap.info"));
    const wasmModule = fs.readFileSync(path.join(wrapDir, "wrap.wasm"));
    const resolverUri = Input.expectUri(resolver.uri);
    const wrap = {
      package: WasmPackage.from(manifest, wasmModule),
      resolverUri,
    };
    config
      .setPackage(wrap.resolverUri.uri, wrap.package)
      .addInterfaceImplementation(
        ExtendableUriResolver.defaultExtInterfaceUris[0].uri,
        resolverUri.uri
      );
  }

  const uri = Input.expectUri(inputObj.uri);
  const client = new PolywrapClient(config.build());

  console.log(`Resolving URI ${uri.uri}`);

  const result = await client.invoke({
    uri: uri.uri,
    method: "",
  });

  const expectedError = Input.expectString(inputObj.expectedError);
  if (!result.ok) {
    console.log(result.error)
    if (result.error?.toString().toLowerCase().includes(expectedError.toLowerCase())) {
      console.log("Expected error received");
    } else {
      console.log(
        `Expected error "${expectedError}", but received "${result.error}"`
      );
    }
  }
}
