import sys
from os import path

import yaml
from flask import Flask, render_template

import config


class WebServer:
    def __init__(self, root_dir, _config: config.Config) -> None:
        self.root_dir = root_dir
        self.config = _config

        self.app = Flask(__name__)
        self.bind_routes()

    def bind_routes(self):
        self.app.route("/")(self.make_index())

    def run(self):
        self.app.run(self.config.addr, self.config.port, debug=True)

    def make_index(self):
        config_dir = path.join(self.root_dir, "config")
        content_path = path.join(config_dir, self.config.content_file)
        content_path = path.abspath(content_path)
        if not path.exists(content_path):
            raise Exception(f"Content file not exists: {content_path}")

        def index():
            with open(content_path, "r", encoding="utf-8") as f:
                content = yaml.load(f, yaml.CFullLoader)
            return render_template("index.html", content=content)

        return index


class Main:
    def __init__(self) -> None:
        self.get_root_dir()
        self.read_config()

        print(f"Root Dir: {self.root_dir}")
        print(f"Final Config: {self.config}")

        self.run_app()

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

    def run_app(self):
        WebServer(self.root_dir, self.config).run()


if __name__ == "__main__":
    Main()
