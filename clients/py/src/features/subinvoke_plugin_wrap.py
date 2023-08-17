from pathlib import Path
from typing import Any, TypedDict

from polywrap import (
    PluginModule,
    PluginPackage,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    Uri,
    WasmPackage,
)
from pydantic import BaseModel, validator

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


class PerformSubinvokeArgs(TypedDict):
    args: Args
    method: str


class Plugin(PluginModule[None]):
    def __init__(self, config: None):
        super().__init__(config)

    def perform_subinvoke(
        self, args: PerformSubinvokeArgs, client: PolywrapClient, *_: Any
    ):
        print(f"Subinvoking {args['method']}")

        try:
            result = client.invoke(
                uri=Uri.from_str("embed/foo"), method=args["method"], args=args["args"]
            )
            print(f"Received: {result}")
            return True
        except Exception as e:
            print(f"Failed to subinvoke method. {e}")
            return False


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

    plugin_package = PluginPackage(Plugin(None), NotImplemented)

    config = (
        PolywrapClientConfigBuilder()
        .set_package(Uri.from_str("embed/foo"), wrap_package)
        .set_package(Uri.from_str("plugin/bar"), plugin_package)
        .build()
    )

    client = PolywrapClient(config)

    print("Invoking Plugin")

    result = client.invoke(
        uri=Uri.from_str("plugin/bar"),
        method="perform_subinvoke",
        args={
            "method": input_obj.method,
            "args": input_obj.args,
        },
    )

    if result:
        print("Success!")
