from pathlib import Path
from typing import Any, TypedDict

from polywrap import PolywrapClient, PolywrapClientConfigBuilder, WasmPackage
from pydantic import BaseModel, Field, validator

from validators import UriStr, validate_root_directory


# Define the Args TypedDict structure
class Args(TypedDict):
    error: str


# Define the input structure as a Pydantic model
class TestCaseInput(BaseModel):
    directory: Path
    uri: UriStr
    method: str
    args: Args
    expected_error: str = Field(..., alias="expectedError")

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


def run_test_case(input: Any) -> None:
    # Parse the input to the defined model
    input_obj = TestCaseInput.parse_obj(input)

    # Identify root directory
    root_wrap_dir = Path(__file__).parent / input_obj.directory

    # Read wrap manifest and wasm module
    manifest_root_path = root_wrap_dir / "wrap.info"
    wasm_root_path = root_wrap_dir / "wrap.wasm"

    with open(manifest_root_path, "rb") as f:
        manifest_root = f.read()

    with open(wasm_root_path, "rb") as f:
        wasm_root_module = f.read()

    # Initialize Wasm package
    root_wrap_package = WasmPackage(
        file_reader=NotImplemented, manifest=manifest_root, wasm_module=wasm_root_module
    )

    # Setup the Polywrap client configuration
    config = (
        PolywrapClientConfigBuilder()
        .set_package(input_obj.uri, root_wrap_package)
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking method {input_obj.method}")

    try:
        client.invoke(
            uri=input_obj.uri,
            method=input_obj.method,
            args=input_obj.args,
        )
    except Exception as error:
        if input_obj.expected_error in str(error):
            print("Expected error received")
        else:
            print(
                f"Expected error {input_obj.expected_error}, but received {str(error)}"
            )
