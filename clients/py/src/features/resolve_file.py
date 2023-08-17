from pathlib import Path
from typing import Any

from polywrap import (
    PolywrapClient,
    PolywrapClientConfigBuilder,
    Uri,
    UriPackage,
    sys_bundle,
)

from validators import validate_root_directory, validate_uri


def run_test_case(input: Any) -> None:
    input_uri = validate_uri(input)
    uri_path = validate_root_directory(
        input_uri.path, Path(__file__).parent.parent.parent.parent.parent
    )
    uri = Uri.from_str(f"{input_uri.authority}/{uri_path}")

    print(f"URI Authority: {uri.authority}")

    config = PolywrapClientConfigBuilder().add_bundle(sys_bundle).build()

    client = PolywrapClient(config)

    print(f"Resolving: {input}")

    result = client.try_resolve_uri(uri)

    match result:
        case UriPackage():
            print("Received: package")
            print("Success!")
        case _:
            raise ValueError("Expected UriPackage")
