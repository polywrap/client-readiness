import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import path from "path";

export async function runTestCase(input: unknown): Promise<void> {
  if (!input || typeof input !== "object") {
    throw Error(
      "wrap_test_harness_map_type test case input must be an object"
    );
  }

  const { map } = input as any;

  if (typeof map !== "object") {
    throw Error("wrap_test_harness_map_type input.map must be an object");
  }

  const root = path.join(__dirname, "../../../../wraps");
  const uri = `fs/${root}/map-type/implementations/as`;

  const config = new ClientConfigBuilder()
    .addDefaults()
    .build();
  const client = new PolywrapClient(config);

  const mapClass = new Map<string, number>();

  for (const entry of Object.entries(map)) {
    mapClass.set(entry[0], entry[1] as number);
  }

  console.log("Invoking returnMap");

  const returnMapResponse1 = await client.invoke<Map<string, number>>({
    uri,
    method: "returnMap",
    args: {
      map: mapClass,
    },
  });

  if (!returnMapResponse1.ok) throw returnMapResponse1.error;

  const returnedMap = returnMapResponse1.value;

  for (const entry of Object.entries(map)) {
    console.log(`key '${entry[0]}' = ${returnedMap.get(entry[0])}`);
  }

  console.log("Success!");
}
