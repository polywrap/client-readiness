import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    rootWrap: unknown;
    subWrap: unknown;
    method: unknown;
    args: unknown;
  }>(input);
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  const verifyWrap = (wrap: unknown, name: string): {
    directory: string;
    uri: Uri;
  } => {
    const wrapObj = Input.expectObject<{
      directory: unknown;
      uri: unknown;
    }>(wrap);

    const directory = Input.expectRootDir(
      wrapObj.directory,
      path.join(__dirname, "../../../../")
    );
    const uri = Input.expectUri(wrapObj.uri);

    return {
      directory,
      uri
    };
  }

  const rootInput = verifyWrap(inputObj.rootWrap, "rootWrap");
  const subInput = verifyWrap(inputObj.subWrap, "subWrap");

  const loadWasmPackage = (wrapDir: string): WasmPackage =>
    WasmPackage.from(
      fs.readFileSync(path.join(wrapDir, "wrap.info")),
      fs.readFileSync(path.join(wrapDir, "wrap.wasm"))
    );

  const rootWrapUri = rootInput.uri;
  const rootWrapPackage = loadWasmPackage(rootInput.directory);

  const subWrapUri = subInput.uri;
  const subWrapPackage = loadWasmPackage(subInput.directory);

  const config = new PolywrapClientConfigBuilder()
    .addPackages({
      [rootWrapUri.uri]: rootWrapPackage,
      [subWrapUri.uri]: subWrapPackage
    })
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  const result = await client.invoke({
    uri: rootWrapUri,
    method,
    args
  });

  if (result.ok) {
    console.log(`Received: ${result.value}`);
    console.log("Success!");
  }
}
