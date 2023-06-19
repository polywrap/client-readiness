from load_feature_specs import load_feature_specs

import os
import sys
import importlib.util


def main():
    # Optional 2nd argument, spec filter_spec
    filter_spec = sys.argv[1] if len(sys.argv) > 1 else None

    specs = load_feature_specs(
        os.path.join(os.path.dirname(__file__), "../../../specs")
    )

    for spec_name, spec in specs.items():
        if filter_spec and filter_spec != spec_name:
            continue

        feature_path = os.path.join(
            os.path.dirname(__file__), f"./features/{spec_name}.py"
        )
        feature_spec = importlib.util.spec_from_file_location(spec_name, feature_path)
        if not feature_spec:
            raise ValueError(f"Invalid feature definition: {feature_path}")

        if not feature_spec.loader:
            raise ValueError(f"Invalid feature definition: {feature_path}")

        feature = importlib.util.module_from_spec(feature_spec)
        feature_spec.loader.exec_module(feature)

        if not hasattr(feature, "run_test_case") or not callable(feature.run_test_case):
            if not spec.required:
                continue
            raise ValueError(f"Invalid feature definition: {feature}")

        print("====================================")
        print(f"Begin Feature Spec Test Cases [{spec_name}]")
        print("====================================")

        test_cases = spec.cases

        for test_case_name, test_case in test_cases.items():
            print(f"$Test Start [{spec_name}.{test_case_name}]")

            try:
                feature.run_test_case(test_case.input)
            except Exception as e:
                print(f"!Test Error [{spec_name}.{test_case_name}]", file=sys.stderr)
                print(e, file=sys.stderr)

        print("====================================")
        print(f"End Feature Spec Test Cases [{spec_name}]")
        print("====================================")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(e)
        sys.exit(1)
    else:
        sys.exit(0)
