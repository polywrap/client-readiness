import { Input } from "../input";

import { PolywrapClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    mainEnv: unknown;
    subinvokerEnv: unknown;
  }>(input);

  const mainEnv = Input.expectObject<Record<string, unknown>>(
    inputObj.mainEnv
  );
  const subinvokerEnv = Input.expectObject<Record<string, unknown>>(
    inputObj.subinvokerEnv
  );

  const wrapsDir = path.join(__dirname, "../../../../wraps");
  const mainPath = path.join(wrapsDir, "/env-type/00-main/implementations/as");
  const mainUri = Uri.from("file/" + mainPath);
  const subinvokerPath = path.join(wrapsDir, "/env-type/02-subinvoker-with-env/implementations/as");
  const subinvokerUri = Uri.from("file/" + subinvokerPath);

  const envs = {
    [mainUri.uri]: mainEnv,
    [subinvokerUri.uri]: subinvokerEnv,
  };

  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .addEnvs(envs)
    .addRedirect("mock/main", mainUri.uri)
    .build();

  const client = new PolywrapClient(config);

  console.log("Invoking methodRequireEnv");

  const methodRequireEnvResult = await client.invoke<{
    str: string
  }>({
    uri: mainUri.uri,
    method: "methodRequireEnv",
    args: {
      arg: "string",
    },
  });

  if (!methodRequireEnvResult.ok) throw methodRequireEnvResult.error;
  console.log(
    "response.str:", methodRequireEnvResult.value.str
  );

  console.log("Success!");

  console.log("Invoking subinvokeMethodRequireEnv");

  const subinvokeEnvMethodResult = await client.invoke<{
    str: string
  }>({
    uri: subinvokerUri.uri,
    method: "subinvokeMethodRequireEnv",
    args: {
      arg: "string",
    },
  });

  if (!subinvokeEnvMethodResult.ok) throw subinvokeEnvMethodResult.error;

  console.log(
    "response.str:", subinvokeEnvMethodResult.value.str
  );
  console.log("Success!")
}
