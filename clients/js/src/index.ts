import { loadFeatureSpecs } from "./load-feature-specs";

import path from "path";

function parseTestCase(filterSpec: string) {
  const filterSpecElem = filterSpec.split(".");
  if (filterSpecElem.length < 2) {
    return [filterSpecElem[0], new Set()];
  }
  return [filterSpecElem[0], new Set(filterSpecElem.slice(1))];
}

function parseFilterSpecs(filterSpecs: string) {
  return Object.fromEntries(
    filterSpecs
      .split(",")
      .filter((x) => x.trim())
      .map(parseTestCase)
  );
}

function setIntersection<T>(setA: Set<T>, setB: Set<T>): Set<T> {
  const intersection = new Set<T>();
  for (const elem of setB) {
    if (setA.has(elem)) {
      intersection.add(elem);
    }
  }
  return intersection;
}


async function main() {
  // Optional 2nd argument, spec filter
  let filter_specs = process.argv.length > 2 ? process.argv[2] : "";
  let filter_specs_map = parseFilterSpecs(filter_specs);
  let filter_specs_set = new Set(Object.keys(filter_specs_map));

  const specs = loadFeatureSpecs(
    path.join(__dirname, "../../../specs")
  );

  let specs_set = new Set(Object.keys(specs));
  let specs_to_run = filter_specs_set.size ? setIntersection(specs_set, filter_specs_set) : specs_set;

  for (const specName of specs_to_run) {
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
    const testCasesSet = new Set(Object.keys(testCases));
    const filterTestCasesSet = filter_specs_map[specName] || new Set();
    const testCasesToRun = filterTestCasesSet.size ? setIntersection(testCasesSet, filterTestCasesSet) : testCasesSet;

    for (const testCase of testCasesToRun) {
      console.log(`$Test Start [${specName}.${testCase}]`);

      try {
        await feature.runTestCase(testCases[testCase].input);
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
