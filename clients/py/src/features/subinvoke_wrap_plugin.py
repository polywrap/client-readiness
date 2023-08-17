from pathlib import Path
from typing import Any, TypedDict

from polywrap import (
    PluginModule,
    PluginPackage,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    WasmPackage,
)
from pydantic import BaseModel, Field, validator

from validators import UriStr, validate_root_directory


class AddArgs(TypedDict):
    a: int
    b: int


class RootWrap(BaseModel):
    directory: Path
    uri: UriStr

    @validator("directory")
    def valid_root_directory(cls, v: Any):
        return validate_root_directory(
            v, Path(__file__).parent.parent.parent.parent.parent
        )


class TestCaseInput(BaseModel):
    root_wrap: RootWrap = Field(..., alias="rootWrap")
    sub_wrap_uri: UriStr = Field(..., alias="subWrapUri")
    method: str
    args: AddArgs


class Plugin(PluginModule[None]):
    def __init__(self, config: None):
        super().__init__(config)

    def add(self, args: AddArgs, *_: Any):
        return args["a"] + args["b"]


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root_wrap_dir = Path(__file__).parent / input_obj.root_wrap.directory

    manifest_path = root_wrap_dir / "wrap.info"
    wasm_path = root_wrap_dir / "wrap.wasm"

    with open(manifest_path, "rb") as f:
        manifest = f.read()

    with open(wasm_path, "rb") as f:
        wasm_module = f.read()

    root_wrap_package = WasmPackage(
        file_reader=NotImplemented, manifest=manifest, wasm_module=wasm_module
    )

    plugin_package = PluginPackage(Plugin(None), NotImplemented)

    config = (
        PolywrapClientConfigBuilder()
        .set_package(input_obj.root_wrap.uri, root_wrap_package)
        .set_package(input_obj.sub_wrap_uri, plugin_package)
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
