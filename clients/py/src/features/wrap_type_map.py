from typing import Any, Dict
from pydantic import BaseModel, Field
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_uri_resolvers import SimpleFileReader, FsUriResolver
from polywrap_core import Uri
from pathlib import Path


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
        raise Exception(f"Error: {response}")

    returned_map = response

    for key in input_obj.map:
        print(f"key '{key}' = {returned_map.get(key)}")

    print("Success!")
