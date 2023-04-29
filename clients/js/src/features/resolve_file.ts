import {
  ClientConfigBuilder,
  PolywrapClient,
  Uri
} from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  if (typeof input !== "string" || !Uri.isValidUri(input)) {
    throw Error(
      "resolve_file test case input must be a string & valid URI"
    );
  }

  const uri = Uri.from(input.replace(
    "$ROOT/",
    path.join(__dirname, "../../../../")
  ));

  console.log(`URI Authority: ${uri.authority}`);

  const config = new ClientConfigBuilder()
    .addDefaults()
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving: ${input}`);

  const result = await client.tryResolveUri({ uri });

  if (result.ok) {
    console.log(`Received: ${result.value.type}`);
    console.log("Success!");
  }
}
