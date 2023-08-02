from typing import Any
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_core import UriPackage
from polywrap_sys_config_bundle import get_sys_config
from validators import validate_uri


def run_test_case(input: Any) -> None:
    uri = validate_uri(input)
    print(f"URI Authority: {uri.authority}")

    config = PolywrapClientConfigBuilder().add(get_sys_config()).build()

    client = PolywrapClient(config)

    print(f"Resolving: {uri.uri}")

    result = client.try_resolve_uri(uri)

    match result:
        case UriPackage():
            print("Received: package")
            print("Success!")
        case _:
            print("Failed!")
