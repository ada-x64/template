from http.server import BaseHTTPRequestHandler, HTTPServer
import subprocess
import os
from pathlib import Path
import json

# echo pepperwepper | sha256sum
pw = "08c505587bc1244785fb5311e8b0498adebe05117a145fe53215c2971db33d14"


class RequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        data = json.loads(body.decode())
        print(data)
        # check auth
        if data["auth"] != pw:
            self.send_response(401)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"Unauthorized\n")
            return
        path = Path(data["path"]).expanduser()
        if os.path.exists(path):
            subprocess.Popen(path)
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
