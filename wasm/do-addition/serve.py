#!/usr/bin/env python3

import http.server
import socketserver

PORT = 8080

Handler = http.server.SimpleHTTPRequestHandler

# Note the one change from the simplest Python file server is to explicitly set the MIME type for .wasm files.
# This makes sure browsers handle these files correctly.
# We make this script executable as well:
# chmod +x serve.py
Handler.extensions_map[".wasm"] = "application/wasm"

httpd = socketserver.TCPServer(("", PORT), Handler)

print("serving at port", PORT)
httpd.serve_forever()