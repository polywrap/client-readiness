from typing import Any, TypeVar, List
from polywrap_client import Uri

T = TypeVar('T')

class Input:
    @staticmethod
    def expect_object(input: Any) -> T:
        if not input or not isinstance(input, dict):
            raise ValueError("expected an object")
        return input

    @staticmethod
    def expect_root_dir(input: str, root_dir: str) -> str:
        if not isinstance(input, str) or "$ROOT/" not in input:
            raise ValueError("expected a string that starts with $ROOT/")
        return input.replace("$ROOT/", root_dir)

    @staticmethod
    def expect_string(input: Any) -> str:
        if not isinstance(input, str):
            raise ValueError("expected a string")
        return input

    @staticmethod
    def expect_uri(input: Any) -> Uri:
        if not isinstance(input, str) or not Uri.is_valid_uri(input):
            raise ValueError("expected a valid WRAP URI")
        return Uri.from_string(input)

    @staticmethod
    def expect_array(input: Any) -> List[T]:
        if not input or not isinstance(input, list):
            raise ValueError("expected an array")
        return input
