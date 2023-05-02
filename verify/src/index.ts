import { loadFeatureSpecs } from "./load-feature-specs";

import path from "path";
import fs from "fs";

async function main() {
  if (process.argv.length < 3) {
    throw Error("No client directory specified.");
  }

  const arg = process.argv[2];
  const cwd = process.cwd();
  const clientDir = path.join(cwd, arg);

  // Optional 2nd argument, spec filter
  const filter = process.argv.length > 3 ? process.argv[3] : undefined;

  // Read stdout & stderr log files
  const stdout = fs.readFileSync(
    path.join(clientDir, "stdout"),
    "utf-8"
  );
  const stderr = fs.readFileSync(
    path.join(clientDir, "stderr"),
    "utf-8"
  );

  // Load Feature Specs
  const featureSpecs = loadFeatureSpecs(
    path.join(__dirname, "../../specs")
  );

  const errors = [];

  for (const featureSpec of Object.keys(featureSpecs)) {
    if (filter && filter !== featureSpec) {
      continue;
    }

    const spec = featureSpecs[featureSpec];
    const cases = spec.cases;

    for (const testCaseName of Object.keys(cases)) {
      const testCase = cases[testCaseName];

      const verify = (
        marker: string,
        stream: "stdout" | "stderr",
        expected: string[],
        received: string
      ): string[] => {
        // Find the first line of output
        const firstLine = expected[0];
        const firstLineIdx = received.indexOf(firstLine);

        if (firstLineIdx < 0) {
          return [
            `Error: ${featureSpec}.${testCaseName} missing ${stream} output "${firstLine}"`
          ];
        }

        let testOutput = received.substring(firstLineIdx);

        // Isolate the test output by removing all future test output
        const nextMarkerIdx = testOutput.indexOf(marker, firstLine.length);

        if (nextMarkerIdx > -1) {
          testOutput = testOutput.substring(
            0, nextMarkerIdx
          );
        }

        // Check the test output for all expected strings
        const errors = [];

        for (let i = 1; i < expected.length; ++i) {
          const expectedIdx = testOutput.indexOf(expected[i]);

          if (expectedIdx < 0) {
            errors.push(
              `Error: ${featureSpec}.${testCaseName} missing ${stream} output "${expected[i]}"`
            );
          }

          testOutput = testOutput.substring(expectedIdx + expected[i].length);
        }

        return errors;
      }

      errors.push(
        ...verify("$Test Start", "stdout", testCase.output.stdout, stdout)
      );

      if (testCase.output.stderr) {
        errors.push(
          ...verify("$Test Start", "stderr", testCase.output.stderr, stderr)
        );
      }
    }
  }

  if (errors.length) {
    throw errors.join("\n");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
