from pathlib import Path
from typing import Any, Dict

from polywrap import (
    FsUriResolver,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    SimpleFileReader,
    Uri,
)
from pydantic import BaseModel


class TestCaseInput(BaseModel):
    map: Dict[str, int]


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    uri = Uri.from_str(f"fs/{root}/map-type/implementations/as")

    config = (
        PolywrapClientConfigBuilder()
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print("Invoking returnMap")

    response = client.invoke(
        uri=uri,
        method="returnMap",
        args={"map": input_obj.map},
    )

    if not response:
        raise ValueError(f"Error: {response}")

    returned_map = response

    for key in input_obj.map:
        print(f"key '{key}' = {returned_map.get(key)}")

    print("Success!")
