import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    method: unknown;
    args: unknown;
  }>(input);
  const method = Input.expectString(inputObj.method);
  const args = Input.expectObject<Record<string, unknown>>(
    inputObj.args
  );

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/enum-type/implementations/as`;

  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .build();
  const client = new PolywrapClient(config);

  console.log(`Invoking ${method}`);

  let response = await client.invoke({
    uri,
    method,
    args: args,
  });

  if (!response.ok) throw response.error;

  console.log("Result:", response.value);
  console.log("Success!");
}
