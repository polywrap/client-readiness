from typing import Any, Callable, List
from polywrap_core import Uri


class Input:
    @staticmethod
    def expect_object(input: Any, validate: Callable[[Any], Any]) -> Any:
        return validate(input)

    @staticmethod
    def expect_root_dir(input: Any, root_dir: Any) -> str:
        if not isinstance(input, str) or "$ROOT/" not in input:
            raise ValueError("expected a string that starts with $ROOT/")

        if not isinstance(root_dir, str):
            raise ValueError("expected a string for root_dir param")

        return input.replace("$ROOT", root_dir)

    @staticmethod
    def expect_string(input: Any) -> str:
        if not isinstance(input, str):
            raise ValueError("expected a string")
        return input

    @staticmethod
    def expect_uri(input: Any) -> Uri:
        if not isinstance(input, str) or not Uri.is_canonical_uri(input):
            raise ValueError("expected a valid WRAP URI")
        return Uri.from_str(input)

    @staticmethod
    def expect_array(input: Any, validate: Callable[[Any], Any]) -> List[Any]:
        if not input or not isinstance(input, list):
            raise ValueError("expected an array")
        return [validate(element) for element in input]
