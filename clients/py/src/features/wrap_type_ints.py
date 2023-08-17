from pathlib import Path
from typing import Any, TypedDict

from polywrap import (
    FsUriResolver,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    SimpleFileReader,
    Uri,
)
from pydantic import BaseModel


class Args(TypedDict):
    first: int
    second: int


class TestCaseInput(BaseModel):
    method: str
    args: Args


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    uri = Uri.from_str(f"fs/{root}/numbers-type/implementations/as")

    config = (
        PolywrapClientConfigBuilder()
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking {input_obj.method}")

    response = client.invoke(
        uri=uri,
        method=input_obj.method,
        args=input_obj.args,
    )

    if not response:
        raise ValueError(f"Error: {response}")

    print("Result:", response)
    print("Success!")
