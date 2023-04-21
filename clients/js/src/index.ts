import { loadFeatureSpecs } from "./load-feature-specs";

import path from "path";

async function main() {
  const specs = loadFeatureSpecs(
    path.join(__dirname, "../../../feature-specs")
  );

  for (const spec of Object.keys(specs)) {
    const feature = await import(`./features/${spec}`);

    if (!feature.runTestCase || typeof feature.runTestCase !== "function") {
      throw Error(`Invalid feature definition: ${feature}`);
    }

    console.log("====================================");
    console.log(`Begin Feature Spec Test Cases [${spec}]`);
    console.log("====================================");

    const testCases = specs[spec];

    for (const testCase of Object.keys(testCases)) {
      console.log(`$Test Start [${spec}.${testCase}]`);

      try {
        feature.runTestCase(testCases[testCase].input);
      } catch (e) {
        console.error(`!Test Error [${spec}.${testCase}]`);
        console.error(e);
      }
    }

    console.log("====================================");
    console.log(`End Feature Spec Test Cases [${spec}]`);
    console.log("====================================");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
