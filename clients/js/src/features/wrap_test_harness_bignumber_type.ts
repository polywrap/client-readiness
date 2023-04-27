import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import BigNumber from "bignumber.js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "wrap_test_harness_bignumber_type test case input must be an object"
    );
  }

  const { args } = input as any;

  if (typeof args !== "object") {
    throw Error("wrap_test_harness_bignumber_type input.args must be an object");
  }

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/bignumber-type/implementations/as`;

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

  const bignumber = new BigNumber(response.value);

  console.log("Result:", bignumber.toString());
  console.log("Success!");
}
