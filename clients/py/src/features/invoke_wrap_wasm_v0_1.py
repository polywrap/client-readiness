from typing import Any, TypedDict
from polywrap_core import Uri
from pydantic import BaseModel, validator
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_wasm import WasmPackage
from pathlib import Path

from validators import validate_root_directory


class Args(TypedDict):
    first: int
    second: int


class TestCaseInput(BaseModel):
    directory: Path
    method: str
    args: Args

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    wrap_dir = Path(__file__).parent / input_obj.directory

    manifest_path = wrap_dir / "wrap.info"
    wasm_path = wrap_dir / "wrap.wasm"

    with open(manifest_path, "rb") as f:
        manifest = f.read()

    with open(wasm_path, "rb") as f:
        wasm_module = f.read()

    wrap_package = WasmPackage(
        file_reader=NotImplemented, manifest=manifest, wasm_module=wasm_module
    )

    config = (
        PolywrapClientConfigBuilder()
        .set_package(Uri.from_str("embed/foo"), wrap_package)
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking {input_obj.method}")

    try:
        result = client.invoke(
            uri=Uri.from_str("embed/foo"), method=input_obj.method, args=input_obj.args
        )
        print(f"Received: {result}")
        print("Success!")
    except Exception as e:
        print(f"Failed to invoke method. {e}")
