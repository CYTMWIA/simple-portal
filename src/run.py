import sys
from os import path

import yaml
from flask import Flask, render_template

import config

app = Flask(__name__)


@app.route("/")
def index():
    with open("config/content.yaml", "r") as f:
        content = yaml.load(f, yaml.CFullLoader)
    return render_template("index.html", content=content)


class Main:
    def __init__(self) -> None:
        self.get_root_dir()
        self.read_config()

        print(f"Root Dir: {self.root_dir}")
        print(f"Final Config: {self.config}")

    def get_root_dir(self):
        root_dir = path.dirname(sys.argv[0])
        root_dir = path.join(root_dir, "../")
        root_dir = path.abspath(root_dir)
        self.root_dir = root_dir

    def read_config(self):
        config_paths = map(
            lambda rp: path.join(self.root_dir, rp),
            ["config/config.json", "config/config-example.json"],
        )
        self.config = config.read(config_paths)


if __name__ == "__main__":
    Main()
