from typing import Any
from polywrap_core import Uri
from input import Input


def run_test_case(input: Any) -> None:
    str_value = Input.expect_string(input)

    uri = Uri.from_str(str_value)

    print("WRAP URI successfully created.")
    print(f"uri - {uri}")
    print(f"uri.authority - {uri.authority}")
    print(f"uri.path - {uri.path}")
