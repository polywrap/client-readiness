from typing import Any

from validators import validate_uri


def run_test_case(input: Any) -> None:
    try:
        uri = validate_uri(input)

        print("WRAP URI successfully created.")
        print(f"uri - {uri}")
        print(f"uri.authority - {uri.authority}")
        print(f"uri.path - {uri.path}")
    except Exception as e:
        raise ValueError(f"Invalid URI Received: {input}") from e
