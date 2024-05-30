from http.server import BaseHTTPRequestHandler,HTTPServer,HTTPStatus
import os

class MyRequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        my_filepath="index.html.gz"
        with open(my_filepath, "rb") as html_gzipped_file:
            content = html_gzipped_file.read()
        self.send_response(HTTPStatus.OK)
        self.send_header("Content-type", "text/html")
        self.send_header("Content-length", os.path.getsize(my_filepath))
        self.send_header("Content-Encoding", "gzip")
        self.end_headers()
        self.wfile.write(content)
        self.wfile.flush()

try:
    server = HTTPServer(('localhost', 22222), MyRequestHandler)
    print('Started http server')
    server.serve_forever()
except KeyboardInterrupt:
    print('^C received, shutting down server')
    server.socket.close()
