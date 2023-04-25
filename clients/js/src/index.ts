import { loadFeatureSpecs } from "./load-feature-specs";

import path from "path";

async function main() {
  const specs = loadFeatureSpecs(
    path.join(__dirname, "../../../specs")
  );

  for (const specName of Object.keys(specs)) {
    const spec = specs[specName];
    const feature = await import(`./features/${specName}`);

    if (!feature.runTestCase || typeof feature.runTestCase !== "function") {
      if (!spec.required) {
        continue;
      }
      throw Error(`Invalid feature definition: ${feature}`);
    }

    console.log("====================================");
    console.log(`Begin Feature Spec Test Cases [${specName}]`);
    console.log("====================================");

    const testCases = specs[specName].cases;

    for (const testCase of Object.keys(testCases)) {
      console.log(`$Test Start [${specName}.${testCase}]`);

      try {
        feature.runTestCase(testCases[testCase].input);
      } catch (e) {
        console.error(`!Test Error [${specName}.${testCase}]`);
        console.error(e);
      }
    }

    console.log("====================================");
    console.log(`End Feature Spec Test Cases [${specName}]`);
    console.log("====================================");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
