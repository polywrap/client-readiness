from pathlib import Path
from typing import Any, TypedDict, Dict

from polywrap import (
    PolywrapClient,
    PolywrapClientConfigBuilder,
    SimpleFileReader,
    Uri,
    WasmPackage,
)
from pydantic import BaseModel, Field, validator
from validators import UriStr, validate_root_directory


# Define the Args TypedDict structure
class Args(TypedDict):
    error: str


# Define the input structure as a Pydantic model
class SubinvokeWrap(BaseModel):
    directory: Path
    uri: UriStr
    method: str
    args: Dict[str, Any]

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


class InvokeWrap(BaseModel):
    directory: Path
    uri: UriStr

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


class InputObj(BaseModel):
    subinvoke_wrap: SubinvokeWrap = Field(..., alias="subinvokeWrap")
    invoke_wrap: InvokeWrap = Field(..., alias="invokeWrap")
    expected_error: str = Field(..., alias="expectedError")


def run_test_case(input: Any) -> None:
    # Parse the input to the defined model
    input_obj = InputObj.parse_obj(input)

    root_dir = Path(__file__).parent

    # Process SubinvokeWrap
    subinvoke_wrap = input_obj.subinvoke_wrap
    subinvoke_wrap_directory = root_dir / subinvoke_wrap.directory

    subinvoke_manifest_path = subinvoke_wrap_directory / "wrap.info"
    subinvoke_wasm_path = subinvoke_wrap_directory / "wrap.wasm"

    with open(subinvoke_manifest_path, "rb") as f:
        subinvoke_manifest = f.read()

    with open(subinvoke_wasm_path, "rb") as f:
        subinvoke_wasm_module = f.read()

    subinvoke_wrap_package = WasmPackage(
        wasm_module=subinvoke_wasm_module,
        file_reader=NotImplemented,
        manifest=subinvoke_manifest,
    )

    # Process InvokeWrap
    invoke_wrap = input_obj.invoke_wrap
    invoke_wrap_directory = root_dir / invoke_wrap.directory

    invoke_manifest_path = invoke_wrap_directory / "wrap.info"
    invoke_wasm_path = invoke_wrap_directory / "wrap.wasm"

    with open(invoke_manifest_path, "rb") as f:
        invoke_manifest = f.read()

    with open(invoke_wasm_path, "rb") as f:
        invoke_wasm_module = f.read()

    invoke_wrap_package = WasmPackage(
        wasm_module=invoke_wasm_module,
        file_reader=NotImplemented,
        manifest=invoke_manifest,
    )

    config = (
        PolywrapClientConfigBuilder()
        .set_package(input_obj.subinvoke_wrap.uri, subinvoke_wrap_package)
        .set_package(input_obj.invoke_wrap.uri, invoke_wrap_package)
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking method {subinvoke_wrap.method}")

    try:
        client.invoke(
            uri=subinvoke_wrap.uri,
            method=subinvoke_wrap.method,
            args=subinvoke_wrap.args,
        )

    except Exception as error:
        if input_obj.expected_error in str(error):
            print("Expected error received")
        else:
            print(
                f"Expected error {input_obj.expected_error}, but received {str(error)}"
            )
