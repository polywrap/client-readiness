import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import BigNumber from "bignumber.js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    args: unknown;
  }>(input);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/bignumber-type/implementations/as`;

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

  const bignumber = new BigNumber(response.value);

  console.log("Result:", bignumber.toString());
  console.log("Success!");
}
