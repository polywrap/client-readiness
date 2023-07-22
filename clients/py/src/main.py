from load_feature_specs import load_feature_specs

import os
import sys
import importlib.util

def parse_test_case(filter_spec):
    filter_spec_elem = filter_spec.split(".")
    if len(filter_spec_elem) < 2:
        return filter_spec_elem[0], set()
    return filter_spec_elem[0], set(filter_spec_elem[1:])

def parse_filter_specs(filter_specs):
    return dict(map(parse_test_case, filter(lambda x: x.strip(), filter_specs.split(","))))


def main():
    # Optional 2nd argument, spec filter_spec
    filter_specs = sys.argv[1] if len(sys.argv) > 1 else None
    filter_specs_map = parse_filter_specs(filter_specs)
    filter_specs_set = set(filter_specs_map.keys())

    specs = load_feature_specs(
        os.path.join(os.path.dirname(__file__), "../../../specs")
    )

    specs_set = set(specs.keys())
    specs_to_run = specs_set.intersection(filter_specs_set) or specs_set

    for spec_name in specs_to_run:
        spec = specs[spec_name]
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

        test_cases_set = set(test_cases.keys())
        filter_test_cases_set = filter_specs_map.get(spec_name, set())

        test_cases_to_run = test_cases_set.intersection(filter_test_cases_set) or test_cases_set

        for test_case_name in test_cases_to_run:
            test_case = test_cases[test_case_name]
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
