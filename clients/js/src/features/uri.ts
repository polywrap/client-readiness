import { Uri } from "@polywrap/client-js";

export function runTestCase(input: unknown): void {
  if (typeof input !== "string") {
    throw Error(
      "URI test case input is not a string"
    );
  }

  const str = input;

  const uri = new Uri(str);

  console.log("WRAP URI successfully created.");
  console.log(`uri - ${uri}`);
  console.log(`uri.authority - ${uri.authority}`);
  console.log(`uri.path - ${uri.path}`);
}
