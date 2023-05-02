import { Input } from "../input";

import {
  ClientConfigBuilder,
  PolywrapClient,
  IUriResolver,
  Uri
} from "@polywrap/client-js";

export async function runTestCase(input: unknown): Promise<void> {
  const inputObj = Input.expectObject<{
    authority: unknown;
    result: unknown;
  }>(input);
  const authority = Input.expectString(inputObj.authority);
  const result = Input.expectString(inputObj.result);

  console.log("Adding Resolver to ClientConfig");

  const resolver: IUriResolver<string> = {
    tryResolveUri: async (uri: Uri) => {
      const response = uri.authority === authority
        ? Uri.from(result)
        : uri;

      return {
        ok: true,
        value: {
          type: "uri",
          uri: response
        }
      };
    }
  }

  const config = new ClientConfigBuilder()
    .addResolver(resolver)
    .build();

  const client = new PolywrapClient(config);

  console.log(`Resolving a wrap://${authority} URI`);

  const res = await client.tryResolveUri({
    uri: Uri.from(`wrap://${authority}/foo`)
  });

  console.log(res)

  if (res.ok && res.value.type === "uri") {
    console.log(`Received URI '${res.value.uri}'`);
    console.log("Success!");
  }
}
