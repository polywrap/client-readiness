import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "resolve_redirect test case input must be an object"
    );
  }

  const { from, to } = input as any;

  if (typeof from !== "string") {
    throw Error(
      "resolve_redirect input.from must be a string"
    );
  }

  if (typeof to !== "string") {
    throw Error(
      "resolve_redirect input.to must be a string"
    );
  }

  const config = new ClientConfigBuilder()
    .addRedirect(from, to)
    .build();

  const client = new PolywrapClient(config);

  console.log("Resolving Redirect");

  const result = await client.tryResolveUri({ uri: from });

  if (result.ok && result.value.type === "uri") {
    console.log(`Received URI '${result.value.uri}'`);
    console.log("Success!");
  }
}
