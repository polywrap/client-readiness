from pathlib import Path
from typing import Any

from polywrap_core import Uri


def validate_root_directory(v: Any, root_dir: Any):
    if isinstance(v, Path):
        v = str(v)

    if not isinstance(v, str) or "$ROOT/" not in v:
        raise ValueError("expected a string that starts with $ROOT/")

    if isinstance(root_dir, Path):
        root_dir = str(root_dir)

    if not isinstance(root_dir, str):
        raise ValueError("expected a string for root_dir param")

    return v.replace("$ROOT", root_dir)


def validate_uri(v: Any) -> Uri:
    if isinstance(v, Uri):
        return v

    if not isinstance(v, str) or not Uri.is_canonical_uri(v):
        raise ValueError("expected a valid WRAP URI")

    return Uri.from_str(v)
