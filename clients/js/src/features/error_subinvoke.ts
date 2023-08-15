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
    subinvokeWrap: {
      directory: unknown;
      uri: unknown;
      method: unknown;
      args: unknown;
    };
    invokeWrap: {
      uri: unknown;
      directory: unknown;
    };
    expectedError: String;
  }>(input);

  // Prepare subinvoke wrap
  const subinvokeWrapDir = Input.expectRootDir(
    inputObj.subinvokeWrap.directory,
    path.join(__dirname, "../../../../")
  );

  const subinvokeWrapManifest = fs.readFileSync(
    path.join(subinvokeWrapDir, "wrap.info")
  );
  const subinvokeWrapWasmModule = fs.readFileSync(
    path.join(subinvokeWrapDir, "wrap.wasm")
  );

  let subinvokeWrapUri = Input.expectUri(inputObj.subinvokeWrap.uri);

  const subinvokeWrap = {
    package: WasmPackage.from(subinvokeWrapManifest, subinvokeWrapWasmModule),
    uri: Uri.from(subinvokeWrapUri.uri),
  };
  const config = new PolywrapClientConfigBuilder();
  config.setPackage(subinvokeWrap.uri.uri, subinvokeWrap.package);

  const invokeWrapDir = Input.expectRootDir(
    inputObj.invokeWrap.directory,
    path.join(__dirname, "../../../../")
  );

  const invokeWrapManifest = fs.readFileSync(
    path.join(invokeWrapDir, "wrap.info")
  );
  const invokeWrapWasmModule = fs.readFileSync(
    path.join(invokeWrapDir, "wrap.wasm")
  );

  let invokeWrapUri = Input.expectUri(inputObj.invokeWrap.uri);
  const invokeWrapPackage = {
    package: WasmPackage.from(invokeWrapManifest, invokeWrapWasmModule),
    uri: Uri.from(invokeWrapUri.uri),
  };
  config.setPackage(invokeWrapPackage.uri.uri, invokeWrapPackage.package);

  const method = Input.expectString(inputObj.subinvokeWrap.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.subinvokeWrap.args
  );

  const client = new PolywrapClient(config.build());

  console.log(`Invoking method ${method}`);

  const result = await client.invoke({
    uri: subinvokeWrapUri.uri,
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
