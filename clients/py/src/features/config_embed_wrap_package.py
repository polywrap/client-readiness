from typing import Any
import sys
from pathlib import Path
from polywrap_core import Uri

from pydantic import BaseModel, validator

from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_client import PolywrapClient
from polywrap_wasm import WasmPackage

from validators import validate_root_directory


class TestCaseInput(BaseModel):
    directory: Path
    method: str
    args: Any

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.validate(input)

    print(f"Reading wrap.info & wrap.wasm from {input_obj.directory}")

    with open(Path(input_obj.directory, "wrap.info"), "rb") as file:
        manifest = file.read()

    with open(Path(input_obj.directory, "wrap.wasm"), "rb") as file:
        wasm_module = file.read()

    print("Creating WrapPackage from raw wrap.info & wrap.wasm bytes")

    wrap_package = WasmPackage(NotImplemented, manifest, wasm_module)

    print("Adding WrapPackage to ClientConfig")

    config = (
        PolywrapClientConfigBuilder()
        .set_package(Uri("embed", "foo"), wrap_package)
        .build()
    )
    client = PolywrapClient(config)

    print("Invoking WrapPackage")

    try:
        client.invoke(
            uri=Uri.from_str("embed/str"), method=input_obj.method, args=input_obj.args
        )
        print("Success!")
    except Exception as e:
        print(e, file=sys.stderr)
