from typing import Any
from polywrap_wasm import WasmPackage
from pydantic import BaseModel, validator
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_manifest import deserialize_wrap_manifest
from polywrap_core import Uri, UriPackage, UriWrapper
from pathlib import Path

from validators import validate_root_directory


class TestCaseInput(BaseModel):
    directory: Path
    uri: str

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent
    wrap_dir = root / input_obj.directory
    uri = Uri.from_str(input_obj.uri)

    with open(wrap_dir / "wrap.info", "rb") as f:
        manifest = deserialize_wrap_manifest(f.read())
    with open(wrap_dir / "wrap.wasm", "rb") as f:
        wasm_module = f.read()

    wrap_package = WasmPackage(
        file_reader=NotImplemented,manifest=manifest, wasm_module=wasm_module
    )

    config = (
        PolywrapClientConfigBuilder()
        .set_package(uri, wrap_package)
        .build()
    )

    client = PolywrapClient(config)

    print(f"Resolving URI: {uri}")

    result = client.try_resolve_uri(uri)

    match result:
        case UriPackage():
            print("Received: UriPackage")
            print("Success!")
        case _:
            print("Failed!")
