import { PolywrapClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(_: unknown): Promise<void> {
  const root = path.join(__dirname, "../../../../wraps");
  const interfaceUri = "wrap://authority/interface";
  const implementationPath = path.join(root, "/interface-invoke/01-implementation/implementations/as");
  const implementationUri = `fs/${implementationPath}`;

  const config = new PolywrapClientConfigBuilder()
    .addDefaults()
    .addInterfaceImplementation(interfaceUri, implementationUri);

  const client = new PolywrapClient(config.build());

  const wrapperPath = path.join(root, "/interface-invoke/02-wrapper/implementations/as");
  const wrapperUri = `fs/${wrapperPath}`;

  console.log("Invoking moduleMethod");

  const result = await client.invoke({
    uri: wrapperUri,
    method: "moduleMethod",
    args: {
      arg: {
        uint8: 1,
        str: "Test String 1",
      },
    },
  });

  if (!result.ok) throw result.error;

  console.log("Success!");
}
