from typing import Any
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_core import Uri, UriPackage
from polywrap_sys_config_bundle import get_sys_config

from validators import validate_uri, validate_root_directory
from pathlib import Path


def run_test_case(input: Any) -> None:
    input_uri = validate_uri(input)
    uri_path = validate_root_directory(input_uri.path, Path(__file__).parent.parent.parent.parent.parent)
    uri = Uri.from_str(f"{input_uri.authority}/{uri_path}")

    print(f"URI Authority: {uri.authority}")

    config = (
        PolywrapClientConfigBuilder()
        .add(get_sys_config())
        .build()
    )

    client = PolywrapClient(config)

    print(f"Resolving: {input}")

    result = client.try_resolve_uri(uri)

    match result:
        case UriPackage():
            print("Received: package")
            print("Success!")
        case _:
            raise ValueError("Expected UriPackage")
