from typing import Any

from polywrap import PolywrapClient, PolywrapClientConfigBuilder, Uri
from pydantic import BaseModel, Field

from validators import UriStr


class TestCaseInput(BaseModel):
    from_uri: UriStr = Field(..., alias="from")
    to_uri: UriStr = Field(..., alias="to")


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    print("Adding URI Redirect to ClientConfig")

    config = (
        PolywrapClientConfigBuilder()
        .set_redirect(input_obj.from_uri, input_obj.to_uri)
        .build()
    )

    client = PolywrapClient(config)

    print("Resolving Redirect")

    res = client.try_resolve_uri(input_obj.from_uri)

    match res:
        case Uri() as uri:
            print(f"Received URI '{uri}'")
            print("Success!")
        case _:
            print("Failed to resolve redirect URI.")
