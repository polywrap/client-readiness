from typing import Any

from polywrap import PolywrapClient, PolywrapClientConfigBuilder, UriPackage, sys_bundle

from validators import validate_uri


def run_test_case(input: Any) -> None:
    uri = validate_uri(input)
    print(f"URI Authority: {uri.authority}")

    config = PolywrapClientConfigBuilder().add_bundle(sys_bundle).build()

    client = PolywrapClient(config)

    print(f"Resolving: {uri.uri}")

    result = client.try_resolve_uri(uri)

    match result:
        case UriPackage():
            print("Received: package")
            print("Success!")
        case _:
            print("Failed!")
