import { Input } from "../input";

import {
  PolywrapClientConfigBuilder,
  PolywrapClient,
  Uri,
  ExtendableUriResolver
} from "@polywrap/client-js";
import {
  PluginPackage
} from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    authority: unknown;
    result: unknown;
  }>(input);
  const authority = Input.expectString(inputObj.authority);
  const result = Input.expectString(inputObj.result);

  console.log("Creating CustomResolverExt Plugin");

  const customResolverExt = {
    uri: "wrap://plugin/custom-resolver",
    plugin: PluginPackage.from(() => ({
      tryResolveUri: async (args: { authority: string }) => {

        if (args.authority === authority) {
          return {
            manifest: null,
            uri: Uri.from(result).uri
          };
        }

        return null;
      }
    }))
  };

  console.log("Adding CustomResolverExt & ExtendableUriResolver to ClientConfig");

  const config = new PolywrapClientConfigBuilder()
    .setPackage(
      customResolverExt.uri,
      customResolverExt.plugin
    )
    .addInterfaceImplementation(
      ExtendableUriResolver.defaultExtInterfaceUris[0].uri,
      customResolverExt.uri
    )
    .addResolver(new ExtendableUriResolver())
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving a wrap://${authority} URI`);

  const res = await client.tryResolveUri({
    uri: Uri.from(`wrap://${authority}/foo`)
  });

  if (res.ok && res.value.type === "uri") {
    console.log(`Received URI '${res.value.uri}'`);
    console.log("Success!");
  }
}
