#!/usr/bin/env -S uv run --script

from http.server import BaseHTTPRequestHandler, HTTPServer
import subprocess
import os
from pathlib import Path
import json
from dataclasses import dataclass
import shlex

# TODO:
# Would be nice to have the following features:
# - Pipe logs back to remote host so they can be viewed in the remote console
#   -> would require a websocket connection, probably
# - Catch signals (i.e. ctrl-c) to close the subprocess
#   -> if piping logs, would want this to be in the remote console
# (when I say "remote console" i mean e.g. the Zed terminal _on the client_
# which is connected to the remote desktop server)

# echo pepperwepper | sha256sum
pw = "08c505587bc1244785fb5311e8b0498adebe05117a145fe53215c2971db33d14"


@dataclass
class RequestBody:
    auth: str
    path: str
    args: str


class RequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        data = json.loads(body.decode())
        data = RequestBody(**data)

        # check auth
        if data.auth != pw:
            self.send_response(401)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"Unauthorized\n")
            return
        path = Path(data.path).expanduser()
        if os.path.exists(path):
            args = shlex.split(data.args)
            print(f"Running {path} {args}")
            subprocess.Popen([path, *args])
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"OK\n")
        else:
            self.send_response(404)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"Not Found\n")


def run(server_class=HTTPServer, port=9995):
    server_address = ("", port)
    httpd = server_class(server_address, RequestHandler)
    print(f"Serving on port {port}...")
    httpd.serve_forever()


run()
