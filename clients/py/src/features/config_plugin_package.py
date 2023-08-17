from typing import Any, TypedDict

from polywrap import (
    PluginModule,
    PluginPackage,
    PolywrapClient,
    PolywrapClientConfigBuilder,
)
from pydantic import BaseModel, validator

from validators import UriStr, validate_uri


class Args(TypedDict):
    a: int
    b: int


class TestCaseInput(BaseModel):
    uri: UriStr
    method: str
    args: Args

    @validator("uri")
    def valid_uri(cls, v: Any):
        return validate_uri(v)


class Plugin(PluginModule[None]):
    def __init__(self, config: None):
        super().__init__(config)

    def add(self, args: Args, *_: Any):
        return args["a"] + args["b"]


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.validate(input)

    print("Creating PluginPackage")

    plugin_package = PluginPackage(Plugin(None), NotImplemented)

    print("Adding PluginPackage to ClientConfig")

    config = (
        PolywrapClientConfigBuilder().set_package(input_obj.uri, plugin_package).build()
    )

    client = PolywrapClient(config)

    print("Invoking PluginPackage")

    try:
        client.invoke(uri=input_obj.uri, method=input_obj.method, args=input_obj.args)
        print("Success!")
    except Exception as e:
        print(e)
