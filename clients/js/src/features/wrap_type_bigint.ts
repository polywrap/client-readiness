import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    args: unknown;
  }>(input);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/bigint-type/implementations/as`;

  const config = new PolywrapClientConfigBuilder()
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
