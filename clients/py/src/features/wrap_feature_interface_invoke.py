from pathlib import Path
from typing import Any

from polywrap import (
    FsUriResolver,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    SimpleFileReader,
    Uri,
)


def run_test_case(_: Any) -> None:
    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    interface_uri = Uri.from_str("wrap://authority/interface")
    implementation_path = root / "interface-invoke/01-implementation/implementations/as"
    implementation_uri = Uri.from_str(f"fs/{implementation_path}")

    config = (
        PolywrapClientConfigBuilder()
        .add_interface_implementations(interface_uri, [implementation_uri])
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    wrapper_path = root / "interface-invoke/02-wrapper/implementations/as"
    wrapper_uri = Uri.from_str(f"fs/{wrapper_path}")

    print("Invoking moduleMethod")

    if result := client.invoke(
        uri=wrapper_uri,
        method="moduleMethod",
        args={
            "arg": {
                "uint8": 1,
                "str": "Test String 1",
            },
        },
    ):
        print("Success!")
    else:
        raise ValueError(f"Error: {result}")
