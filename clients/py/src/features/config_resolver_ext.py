from typing import Any, TypedDict
from polywrap_core import Uri
from pydantic import BaseModel
import sys
from polywrap_client import PolywrapClient
from polywrap_uri_resolvers import ExtendableUriResolver
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_plugin import PluginModule, PluginPackage

from validators import UriStr


class Args(TypedDict):
    authority: str


class Plugin(PluginModule[None]):
    def __init__(self, config: None, authority: str, result: UriStr):
        super().__init__(config)
        self.authority = authority
        self.result = result
        setattr(self, "tryResolveUri", self.try_resolve_uri)

    def try_resolve_uri(self, args: Args, *_: Any):
        if args["authority"] == self.authority:
            return {"manifest": None, "uri": self.result.uri}
        return None


class TestCaseInput(BaseModel):
    authority: str
    result: UriStr


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    print("Creating CustomResolverExt Plugin")

    plugin = Plugin(None, input_obj.authority, input_obj.result)

    custom_resolver_ext_uri = Uri.from_str("wrap://plugin/custom-resolver")
    custom_resolver_ext_plugin = PluginPackage(plugin, NotImplemented)

    print("Adding CustomResolverExt & ExtendableUriResolver to ClientConfig")

    config = (
        PolywrapClientConfigBuilder()
        .set_package(custom_resolver_ext_uri, custom_resolver_ext_plugin)
        .add_interface_implementations(
            ExtendableUriResolver.DEFAULT_EXT_INTERFACE_URIS[0],
            [custom_resolver_ext_uri],
        )
        .add_resolver(ExtendableUriResolver())
        .build()
    )

    client = PolywrapClient(config)

    print(f"Resolving a wrap://{input_obj.authority} URI")

    res = client.try_resolve_uri(Uri.from_str(f"wrap://{input_obj.authority}/foo"))

    match res:
        case Uri() as uri:
            print(f"Received URI '{uri}'")
            print("Success!")
        case _:
            print("Failed to resolve URI.", file=sys.stderr)
