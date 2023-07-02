from typing import Any, TypedDict
from pydantic import BaseModel, Field, validator
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_wasm import WasmPackage
from pathlib import Path

from validators import UriStr, validate_root_directory


class Args(TypedDict):
    a: int
    b: int


class Wrap(BaseModel):
    directory: Path
    uri: UriStr

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


class TestCaseInput(BaseModel):
    root_wrap: Wrap = Field(..., alias="rootWrap")
    sub_wrap: Wrap = Field(..., alias="subWrap")
    method: str
    args: Args


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root_wrap_dir = Path(__file__).parent / input_obj.root_wrap.directory
    sub_wrap_dir = Path(__file__).parent / input_obj.sub_wrap.directory

    manifest_root_path = root_wrap_dir / "wrap.info"
    wasm_root_path = root_wrap_dir / "wrap.wasm"

    manifest_sub_path = sub_wrap_dir / "wrap.info"
    wasm_sub_path = sub_wrap_dir / "wrap.wasm"

    with open(manifest_root_path, "rb") as f:
        manifest_root = f.read()

    with open(wasm_root_path, "rb") as f:
        wasm_root_module = f.read()

    with open(manifest_sub_path, "rb") as f:
        manifest_sub = f.read()

    with open(wasm_sub_path, "rb") as f:
        wasm_sub_module = f.read()

    root_wrap_package = WasmPackage(
        file_reader=NotImplemented, manifest=manifest_root, wasm_module=wasm_root_module
    )
    sub_wrap_package = WasmPackage(
        file_reader=NotImplemented, manifest=manifest_sub, wasm_module=wasm_sub_module
    )

    config = (
        PolywrapClientConfigBuilder()
        .set_package(input_obj.root_wrap.uri, root_wrap_package)
        .set_package(input_obj.sub_wrap.uri, sub_wrap_package)
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking {input_obj.method}")

    result = client.invoke(
        uri=input_obj.root_wrap.uri,
        method=input_obj.method,
        args=input_obj.args,
    )

    if result:
        print(f"Received: {result}")
        print("Success!")
