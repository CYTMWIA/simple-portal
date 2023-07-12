import json
import os
from dataclasses import dataclass, field

from collections.abc import Iterable


@dataclass
class Config:
    addr: str = "0.0.0.0"
    port: int = 11088
    content_file: str = "./content.yaml"
    api_key: list[str] = field(default_factory=list)


def read(path) -> dict:
    if isinstance(path, str):
        paths = [path]
    elif isinstance(path, Iterable):
        paths = list(path)
    else:
        raise Exception(f"Unknown type of path: {path}")

    dct = {}
    for i in range(len(paths) - 1, -1, -1):  # 位于前面的路径具有更高的优先级
        file_path = paths[i]
        if not os.path.exists(file_path):
            print(f"File not exists: {file_path}")
            continue
        with open(file_path, "r", encoding="utf-8") as f:
            dct.update(json.load(f))

    cfg = Config(**dct)

    return cfg


if __name__ == "__main__":
    pass  # Test
    print(read("config/config-example.json"))
