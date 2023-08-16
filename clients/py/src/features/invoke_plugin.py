from typing import Any, TypedDict

from polywrap import (
    PluginModule,
    PluginPackage,
    PolywrapClient,
    PolywrapClientConfigBuilder,
)
from pydantic import BaseModel

from validators import UriStr


class Args(TypedDict):
    a: int
    b: int


class TestCaseInput(BaseModel):
    uri: UriStr
    method: str
    args: Args


class Plugin(PluginModule[None]):
    def __init__(self, config: None):
        super().__init__(config)

    def add(self, args: Args, *_: Any):
        return args["a"] + args["b"]


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    plugin_package = PluginPackage(Plugin(None), NotImplemented)

    config = (
        PolywrapClientConfigBuilder().set_package(input_obj.uri, plugin_package).build()
    )

    client = PolywrapClient(config)

    print(f"Invoking {input_obj.method}")

    try:
        result = client.invoke(
            uri=input_obj.uri, method=input_obj.method, args=input_obj.args
        )
        print(f"Received: {result}")
        print("Success!")
    except Exception as e:
        print(f"Failed to invoke method. {e}")
