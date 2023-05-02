import fs from "fs";
import path from "path";
import yaml from "yaml";

export interface TestCases {
  [testCase: string]: {
    input: unknown;
  }
}

export interface Spec {
  required: boolean;
  cases: TestCases;
}

export interface FeatureSpecs {
  [spec: string]: Spec;
}

export function loadFeatureSpecs(directory: string): FeatureSpecs {
  const featureSpecs: FeatureSpecs = { };

  const specFiles = fs.readdirSync(
    directory
  );

  for (const specFile of specFiles) {
    const specYaml = fs.readFileSync(
      path.join(directory, specFile),
      "utf-8"
    );

    const spec = yaml.parse(specYaml);

    if (!spec || typeof spec !== "object") {
      throw Error(`Failed to load feature-spec ${specFile}, must be an object.`);
    }

    const required = spec.required;

    if (typeof required !== "boolean") {
      throw Error(`Failed to load feature-spec ${specFile}, must have property 'required'.`);
    }

    const cases = spec.cases;

    if (typeof cases !== "object") {
      throw Error(`Failed to load feature-spec ${specFile}, must have property 'cases'.`);
    }

    for (const testCase of Object.keys(cases)) {
      if (!cases[testCase].input) {
        throw Error(
          `Failed to load feature spec test case ${specFile}.cases.${testCase}, missing 'input' property`
        );
      }
    }

    const specName = specFile.replace(
      path.extname(specFile),
      ""
    );

    featureSpecs[specName] = spec;
  }

  return featureSpecs;
}
