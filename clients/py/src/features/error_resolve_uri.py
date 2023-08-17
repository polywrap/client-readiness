from pathlib import Path
from typing import Any, Optional
from polywrap import (
    PolywrapClient,
    PolywrapClientConfigBuilder,
    SimpleFileReader,
    ExtendableUriResolver,
    WasmPackage,
)
from pydantic import BaseModel, Field, validator
from validators import UriStr, validate_root_directory

# Define the WrapDir structure as a Pydantic model
class WrapDir(BaseModel):
    directory: Path
    uri: UriStr

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )

# Define the input structure as a Pydantic model
class InputObj(BaseModel):
    resolver: Optional[WrapDir]
    uri: UriStr
    expected_error: str = Field(..., alias="expectedError")


def run_test_case(input: Any) -> None:
    # Parse the input to the defined model
    input_obj = InputObj.parse_obj(input)

    root_dir = Path(__file__).parent.parent.parent.parent.parent

    config_builder = PolywrapClientConfigBuilder()

    if input_obj.resolver:
        # Process resolver WrapDir
        resolver_directory = root_dir / input_obj.resolver.directory

        resolver_manifest_path = resolver_directory / "wrap.info"
        resolver_wasm_path = resolver_directory / "wrap.wasm"

        with open(resolver_manifest_path, "rb") as f:
            resolver_manifest = f.read()

        with open(resolver_wasm_path, "rb") as f:
            resolver_wasm_module = f.read()

        resolver_package = WasmPackage(
            wasm_module=resolver_wasm_module,
            file_reader=SimpleFileReader(),
            manifest=resolver_manifest,
        )

        config_builder.set_package(input_obj.resolver.uri, resolver_package)
        config_builder.add_interface_implementations(
            ExtendableUriResolver.DEFAULT_EXT_INTERFACE_URIS[0], [input_obj.resolver.uri]
        )

    config = config_builder.build()

    client = PolywrapClient(config)

    print(f"Resolving URI {input_obj.uri}")

    try:
        client.invoke(uri=input_obj.uri, method="", args=None)

    except Exception as error:
        if input_obj.expected_error in str(error):
            print("Expected error received")
        else:
            print(
                f"Expected error {input_obj.expected_error}, but received {str(error)}"
            )
