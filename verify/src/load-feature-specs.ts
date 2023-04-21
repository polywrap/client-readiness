import fs from "fs";
import path from "path";
import yaml from "yaml";

export interface TestCases {
  [testCase: string]: {
    output: {
      stdout: string[];
      stderr?: string[];
    }
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
      if (!spec[testCase].output) {
        throw Error(
          `Failed to load feature-spec test-case ${specFile}.${testCase}, missing 'output' property`
        );
      }

      const output = spec[testCase].output;

      if (typeof output !== "object") {
        throw Error(
          `Test case ${specFile}.${testCase} 'output' property must be an object`
        );
      }

      if (!output.stdout || !Array.isArray(output.stdout)) {
        throw Error(
          `Test case ${specFile}.${testCase} 'output.stdout' property must be an array`
        );
      }

      if (output.stderr && !Array.isArray(output.stderr)) {
        throw Error(
          `Test case ${specFile}.${testCase} 'output.stderr' property must be an array`
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
