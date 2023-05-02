import { Uri } from "@polywrap/client-js";

export const Input = {
  expectObject: <T>(input: unknown): T => {
    if (!input || typeof input !== "object") {
      throw Error("expected an object");
    }
    return input as T;
  },
  expectRootDir: (input: unknown, rootDir: string): string => {
    if (typeof input !== "string" || !input.includes("$ROOT/")) {
      throw Error("expected a string that starts with $ROOT/");
    }
    return input.replace("$ROOT/", rootDir);
  },
  expectString: (input: unknown): string => {
    if (typeof input !== "string") {
      throw Error("expected a string");
    }
    return input;
  },
  expectUri: (input: unknown): Uri => {
    if (typeof input !== "string" || !Uri.isValidUri(input)) {
      throw Error("expected a valid WRAP URI");
    }
    return Uri.from(input);
  },
  expectArray: <T>(input: unknown): Array<T> => {
    if (!input || !Array.isArray(input)) {
      throw Error("expected an array");
    }
    return input as Array<T>;
  }
}
