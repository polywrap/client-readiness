from typing import Any, List, cast
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_core import Uri

from pydantic import BaseModel, Field

from validators import UriStr


class TestCaseInput(BaseModel):
    interface: UriStr = Field(..., alias="interfaceUri")
    implementations: List[UriStr]


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.validate(input)

    print("Adding Interface Implementations to ClientConfig")

    config = (
        PolywrapClientConfigBuilder()
        .add_interface_implementations(
            input_obj.interface, cast(List[Uri], input_obj.implementations)
        )
        .build()
    )
    client = PolywrapClient(config)

    print("Getting Implementations")

    try:
        result = client.get_implementations(input_obj.interface)
        if result and len(result) > 0:
            print(f"Found {len(result)} Implementations")
            print("Success!")
    except Exception as e:
        print(e)
