import fs from "fs";
import path from "path";
import yaml from "yaml";

export interface TestCases {
  [testCase: string]: {
    input: unknown;
  }
}

export interface FeatureSpecs {
  [spec: string]: TestCases;
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

    if (typeof spec !== "object") {
      throw Error(`Failed to load feature-spec ${specFile}, must be an object.`);
    }

    for (const testCase of Object.keys(spec)) {
      if (!spec[testCase].input) {
        throw Error(
          `Failed to load feature-spec test-case ${specFile}.${testCase}, missing 'input' property`
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
