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
    external_wrapper_path = root / "env-type/00-external/implementations/as"
    external_wrapper_uri = Uri.from_str(f"fs/{external_wrapper_path}")

    wrapper_path = root / "env-type/01-main/implementations/as"
    wrapper_uri = Uri.from_str(f"fs/{wrapper_path}")

    subinvoker_path = root / "env-type/02-subinvoker-with-env/implementations/as"
    subinvoker_uri = Uri.from_str(f"fs/{subinvoker_path}")

    envs = {
        wrapper_uri: input_obj.main_env,
        external_wrapper_uri: input_obj.subinvoker_env,
    }

    config = (
        PolywrapClientConfigBuilder()
        .set_envs(envs)
        .set_redirect(
            Uri.from_str("ens/external-env.polywrap.eth"), external_wrapper_uri
        )
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print("Invoking methodRequireEnv")

    method_require_env_result = client.invoke(
        uri=wrapper_uri,
        method="methodRequireEnv",
        args={
            "arg": "string",
        },
    )

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

    print(subinvoke_env_method_result)

    if not subinvoke_env_method_result:
        raise Exception(f"Error: {subinvoke_env_method_result}")

    print("response.str:", subinvoke_env_method_result.get("str"))
    print("Success!")
