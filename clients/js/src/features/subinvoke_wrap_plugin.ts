import { ClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import { WasmPackage } from "@polywrap/wasm-js";
import { PluginPackage } from "@polywrap/plugin-js";

import path from "path";
import fs from "fs";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "subinvoke_wrap_plugin test case input must be an object"
    );
  }

  const { rootWrap, subWrapUri, method, args } = input as any;

  const verifyWrap = (wrap: unknown, name: string): {
    directory: string;
    uri: string;
  } => {
    if (!wrap || typeof wrap !== "object") {
      throw Error(
        `subinvoke_wrap_plugin input.${name} must be an object`
      );
    }

    const { directory, uri } = wrap as any;

    if (
      typeof directory !== "string" ||
      !directory.includes("$ROOT/")
    ) {
      throw Error(
        `subinvoke_wrap_plugin input.${name}.directory must be a string, and start with $ROOT/`
      );
    }

    if (!Uri.isValidUri(uri)) {
      throw Error(
        `subinvoke_wrap_plugin input.${name}.uri must be a valid URI`
      );
    }

    return {
      directory,
      uri
    };
  }

  const rootInput = verifyWrap(rootWrap, "rootWrap");

  if (typeof subWrapUri !== "string" || !Uri.isValidUri(subWrapUri)) {
    throw Error("subinvoke_wrap_plugin input.subWrapUri must be a valid URI");
  }

  if (typeof method !== "string") {
    throw Error("subinvoke_wrap_plugin input.method must be a string");
  }

  if (typeof args !== "object") {
    throw Error("subinvoke_wrap_wrap input.args must be an object");
  }

  const loadWasmPackage = (dir: string, uri: string): {
    package: WasmPackage;
    uri: Uri;
  } => {
    const wrapDir = dir.replace(
      "$ROOT/",
      path.join(__dirname, "../../../../")
    );
    const manifest = fs.readFileSync(
      path.join(wrapDir, "wrap.info")
    );
    const wasmModule = fs.readFileSync(
      path.join(wrapDir, "wrap.wasm")
    );
    return {
      package: WasmPackage.from(manifest, wasmModule),
      uri: Uri.from(uri)
    };
  }

  const root = loadWasmPackage(rootInput.directory, rootInput.uri);
  const sub = {
    package: PluginPackage.from(() => ({
      add: (args: { a: number, b: number }) => {
        return args.a + args.b;
      }
    })),
    uri: Uri.from(subWrapUri)
  };

  const config = new ClientConfigBuilder()
    .addPackages({
      [root.uri.uri]: root.package,
      [sub.uri.uri]: sub.package
    })
    .build();

  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  const result = await client.invoke({
    uri: root.uri,
    method,
    args
  });

  if (result.ok) {
    console.log(`Received: ${result.value}`);
    console.log("Success!");
  }
}
