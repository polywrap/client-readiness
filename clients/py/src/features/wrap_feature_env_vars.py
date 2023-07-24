from typing import Any, Dict, Optional, TypedDict, List
from pydantic import BaseModel, Field
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_uri_resolvers import SimpleFileReader, FsUriResolver
from polywrap_core import Uri
from pathlib import Path

Env = TypedDict(
    "Env",
    {
        "str": str,
        "object": Any,
        "array": List[int],
        "number": int,
        "bool": bool,
        "en": str,
        "optFilledStr": Optional[str],
    },
    total=False,
)

ExtEnv = TypedDict(
    "ExtEnv",
    {
        "externalArray": List[int],
        "externalString": str,
    },
    total=False,
)


class TestCaseInput(BaseModel):
    main_env: Env = Field(..., alias="mainEnv")
    subinvoker_env: Env = Field(..., alias="subinvokerEnv")


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"

    main_path = root / "env-type/00-main/implementations/as"
    main_uri = Uri.from_str(f"fs/{main_path}")

    subinvoker_path = root / "env-type/02-subinvoker-with-env/implementations/as"
    subinvoker_uri = Uri.from_str(f"fs/{subinvoker_path}")

    envs = {
        main_uri: input_obj.main_env,
        subinvoker_uri: input_obj.subinvoker_env,
    }

    config = (
        PolywrapClientConfigBuilder()
        .set_envs(envs)
        .set_redirect(
            Uri.from_str("mock/main"), main_uri
        )
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print("Invoking methodRequireEnv")

    try:
        method_require_env_result = client.invoke(
            uri=main_uri,
            method="methodRequireEnv",
            args={
                "arg": "string",
            },
        )
    except Exception as e:
        print("Error:", e)
        raise e

    if not method_require_env_result:
        raise Exception(f"Error: {method_require_env_result}")

    print("response.str:", method_require_env_result.get("str"))
    print("Success!")

    print("Invoking subinvokeMethodRequireEnv")

    print(subinvoker_uri)

    subinvoke_env_method_result = client.invoke(
        uri=subinvoker_uri,
        method="subinvokeMethodRequireEnv",
        args={
            "arg": "string",
        },
    )

    if not subinvoke_env_method_result:
        raise Exception(f"Error: {subinvoke_env_method_result}")

    print("response.str:", subinvoke_env_method_result.get("str"))
    print("Success!")
