import { ClientConfigBuilder, PolywrapClient, Uri } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "wrap_test_harness_env_type test case input must be an object"
    );
  }

  const { mainEnv, extEnv } = input as any;

  if (typeof mainEnv !== "object") {
    throw Error("wrap_test_harness_env_type input.mainEnv must be an object");
  }

  if (typeof extEnv !== "object") {
    throw Error("wrap_test_harness_env_type input.extEnv must be an object");
  }

  const root = path.join(__dirname, "../../../../wraps");
  const externalWrapperPath = path.join(root, "/env-type/00-external/implementations/as");
  const { uri: externalWrapperUri } = Uri.from(
    `file/${externalWrapperPath}`
  );

  const wrapperPath = path.join(root, "/env-type/01-main/implementations/as");
  const { uri: wrapperUri } = Uri.from(`file/${wrapperPath}`);

  const envs = {
    [wrapperUri]: mainEnv,
    [externalWrapperUri]: extEnv,
  };

  const config = new ClientConfigBuilder()
    .addDefaults()
    .addEnvs(envs)
    .addRedirect("ens/external-env.polywrap.eth", externalWrapperUri)
    .build();

  const client = new PolywrapClient(config);

  console.log("Invoking methodRequireEnv");

  const methodRequireEnvResult = await client.invoke({
    uri: wrapperUri,
    method: "methodRequireEnv",
    args: {
      arg: "string",
    },
  });

  if (!methodRequireEnvResult.ok) throw methodRequireEnvResult.error;

  console.log("Success!");

  console.log("Invoking subinvokeEnvMethod");

  const subinvokeEnvMethodResult = await client.invoke<{
    local: unknown,
    external: unknown
  }>({
    uri: wrapperUri,
    method: "subinvokeEnvMethod",
    args: {
      arg: "string",
    },
  });

  if (!subinvokeEnvMethodResult.ok) throw subinvokeEnvMethodResult.error;

  console.log(
    "response.local exists:", !!subinvokeEnvMethodResult.value.local
  );
  console.log(
    "response.external exists:", !!subinvokeEnvMethodResult.value.external
  );
  console.log("Success!")
}
