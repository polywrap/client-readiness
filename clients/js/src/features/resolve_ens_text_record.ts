import {
  ClientConfigBuilder,
  PolywrapClient,
  Uri
} from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  if (typeof input !== "string" || !Uri.isValidUri(input)) {
    throw Error(
      "resolve_ens_text_record test case input must be a string & valid URI"
    );
  }

  const uri = Uri.from(input);

  console.log(`URI Authority: ${uri.authority}`);

  const config = new ClientConfigBuilder()
    .addDefaults()
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving: ${uri}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
