import {
  ClientConfigBuilder,
  PolywrapClient,
  Uri,
  ExtendableUriResolver
} from "@polywrap/client-js";
import {
  PluginPackage
} from "@polywrap/plugin-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "config_resolver_ext test case input must be an object"
    );
  }

  const { authority, result } = input as any;

  if (typeof authority !== "string") {
    throw Error(
      "config_resolver_ext input.authority must be a string"
    );
  }

  if (typeof result !== "string") {
    throw Error(
      "config_resolver_ext input.result must be a string"
    );
  }

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

  const config = new ClientConfigBuilder()
    .addPackage(
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
