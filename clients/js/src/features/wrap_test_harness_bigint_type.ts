import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "wrap_test_harness_bigint_type test case input must be an object"
    );
  }

  const { args } = input as any;

  if (typeof args !== "object") {
    throw Error("wrap_test_harness_bigint_type input.args must be an object");
  }

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/bigint-type/implementations/as`;

  const config = new ClientConfigBuilder()
    .addDefaults()
    .build();
  const client = new PolywrapClient(config);

  console.log("Invoking method");

  let response = await client.invoke<string>({
    uri,
    method: "method",
    args: args,
  });

  if (!response.ok) throw response.error;

  const bigint = BigInt(response.value);

  console.log("Result:", bigint.toString());
  console.log("Success!");
}
