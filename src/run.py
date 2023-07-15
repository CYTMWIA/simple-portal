import hashlib
import os
import shutil
import sys

import yaml
from flask import Flask, abort, render_template, request

import config


def sha3_256(s: str):
    encode = s.strip().encode("utf-8")
    return hashlib.sha3_256(encode).digest().hex()


class WebServer:
    def __init__(self, root_dir, _config: config.Config) -> None:
        self.root_dir = root_dir
        self.config = _config

        self.make_paths()

        self.app = Flask(__name__)
        self.bind_routes()

    def make_paths(self):
        config_dir = os.path.join(self.root_dir, "config")
        self.content_path = os.path.join(config_dir, self.config.content_file)

    def bind_routes(self):
        self.app.route("/")(self.__index())
        self.app.route("/content", methods=["POST"])(self.__content())

    def run(self):
        self.app.run(self.config.addr, self.config.port, debug=True)

    def __index(self):
        def index():
            with open(self.content_path, "r", encoding="utf-8") as f:
                content = yaml.load(f, yaml.CFullLoader)
            return render_template("index.html", content=content)

        return index

    def __content(self):
        def content():
            password = request.headers.get("Authorization")
            if password == None:
                abort(401)
            hashed = sha3_256(password)
            if hashed not in self.config.api_key:
                abort(403)

            print(request.files)
            if "file" not in request.files:
                abort(400)
            file = request.files["file"]
            # If the user does not select a file, the browser submits an
            # empty file without a filename.
            if file.filename == "":
                abort(400)

            if file:
                file.save(self.content_path)
                return ("", 200)

        return content


class Main:
    def __init__(self) -> None:
        self.get_root_dir()
        self.read_config()

        print(f"Root Dir: {self.root_dir}")
        print(f"Final Config: {self.config}")

        self.run_app()

    def get_root_dir(self):
        root_dir = os.path.dirname(sys.argv[0])
        root_dir = os.path.join(root_dir, "../")
        root_dir = os.path.abspath(root_dir)
        self.root_dir = root_dir

    def read_config(self):
        config_paths = map(
            lambda rp: os.path.join(self.root_dir, rp),
            ["config/config.json", "config/config-example.json"],
        )
        self.config = config.read(config_paths)

    def run_app(self):
        WebServer(self.root_dir, self.config).run()


if __name__ == "__main__":
    Main()
