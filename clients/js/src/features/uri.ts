import { Input } from "../input";

import { Uri } from "@polywrap/client-js";

export function runTestCase(input: unknown): void {
  const str = Input.expectString(input);

  const uri = new Uri(str);

  console.log("WRAP URI successfully created.");
  console.log(`uri - ${uri}`);
  console.log(`uri.authority - ${uri.authority}`);
  console.log(`uri.path - ${uri.path}`);
}
