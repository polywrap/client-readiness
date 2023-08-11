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
    invokeWrap: {
      directory: unknown;
      uri: unknown;
      method: unknown;
      args: unknown;
    };
    subinvokeWrap: {
      uri: unknown;
      directory: unknown;
    };
  }>(input);
  const invokeWrapDir = Input.expectRootDir(
    inputObj.invokeWrap.directory,
    path.join(__dirname, "../../../../")
  );

  const subinvokeWrapDir = Input.expectRootDir(
    inputObj.subinvokeWrap.directory,
    path.join(__dirname, "../../../../")
  );

  const method = Input.expectString(inputObj.invokeWrap.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.invokeWrap.args
  );

  const invokeWrapManifest = fs.readFileSync(
    path.join(invokeWrapDir, "wrap.info")
  );
  const invokeWrapWasmModule = fs.readFileSync(
    path.join(invokeWrapDir, "wrap.wasm")
  );

  const subinvokeWrapManifest = fs.readFileSync(
    path.join(subinvokeWrapDir, "wrap.info")
  );
  const subinvokeWrapWasmModule = fs.readFileSync(
    path.join(subinvokeWrapDir, "wrap.wasm")
  );

  let invokeWrapUri = Input.expectUri(inputObj.invokeWrap.uri);

  const invokeWrap = {
    package: WasmPackage.from(invokeWrapManifest, invokeWrapWasmModule),
    uri: Uri.from(invokeWrapUri.uri),
  };

  let subinvokeWrapUri = Input.expectUri(inputObj.subinvokeWrap.uri);
  const subinvokeWrapPackage = {
    package: WasmPackage.from(subinvokeWrapManifest, subinvokeWrapWasmModule),
    uri: Uri.from(subinvokeWrapUri.uri),
  };

  const config = new PolywrapClientConfigBuilder()
    .setPackage(invokeWrap.uri.uri, invokeWrap.package)
    .setPackage(subinvokeWrapPackage.uri.uri, subinvokeWrapPackage.package)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking method ${method}`);

  const result = await client.invoke({
    uri: invokeWrapUri.uri,
    method,
    args,
  });

  if (!result.ok) {
    console.log("Received error: " + result.error?.innerError?.reason);
  }
}
