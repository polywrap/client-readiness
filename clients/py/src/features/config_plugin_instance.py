from typing import Any
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_plugin import PluginModule, PluginWrapper

from pydantic import BaseModel

from validators import UriStr


class TestCaseInput(BaseModel):
    uri: UriStr
    method: str


class Plugin(PluginModule[None]):
    counter = 0

    def __init__(self, config: None):
        super().__init__(config)

    def increment(self, *_: Any):
        self.counter += 1


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.validate(input)

    print("Creating Plugin Instance")

    plugin = Plugin(None)

    print("Adding Plugin Instance to ClientConfig")

    plugin_wrapper = PluginWrapper(plugin, NotImplemented)

    config = (
        PolywrapClientConfigBuilder().set_wrapper(input_obj.uri, plugin_wrapper).build()
    )

    client = PolywrapClient(config)

    for _ in range(2):
        print("Invoking Plugin Instance")

        try:
            client.invoke(uri=input_obj.uri, method=input_obj.method)

            print(f"counter = {plugin.counter}")
        except Exception as e:
            print(e)

    print("Success!")
