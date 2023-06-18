from typing import Any

from validators import validate_uri


def run_test_case(input: Any) -> None:
    uri = validate_uri(input)

    print("WRAP URI successfully created.")
    print(f"uri - {uri}")
    print(f"uri.authority - {uri.authority}")
    print(f"uri.path - {uri.path}")
