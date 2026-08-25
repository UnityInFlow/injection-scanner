#!/usr/bin/env python3
"""Local prompt-injection lab.

Serves a corpus of documents that each attempt one injection technique in one
markdown context, then lets you measure two independent things:

  * whether an AI agent that reads the page acts on the payload (canary tokens)
  * whether injection-scanner reports it (run the CLI over corpus/)

Localhost only. Every payload asks for a harmless canary token -- nothing here
touches the network, the filesystem, or any real system.
"""
import http.server, json, pathlib, socketserver, html, datetime, sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8137
ROOT = pathlib.Path(__file__).parent
CORPUS = ROOT / "corpus"
MANIFEST = json.loads((ROOT / "manifest.json").read_text())
ACCESS_LOG = ROOT / "access.log"


def render(md: str) -> str:
    """Minimal markdown -> HTML.

    Deliberately preserves HTML comments verbatim: a comment that survives into
    the DOM is invisible to a human reviewer and fully visible to a model that
    reads page source, which is the entire point of document 03.
    """
    out, in_fence = [], False
    for line in md.split("\n"):
        if line.startswith("```") or line.startswith("~~~"):
            out.append("</pre>" if in_fence else "<pre>")
            in_fence = not in_fence
            continue
        if in_fence:
            out.append(html.escape(line))
        elif line.startswith("<!--"):
            out.append(line)                      # passed through as-is
        elif line.startswith("# "):
            out.append(f"<h1>{html.escape(line[2:])}</h1>")
        elif line.startswith("## "):
            out.append(f"<h2>{html.escape(line[3:])}</h2>")
        elif line.strip():
            out.append(f"<p>{html.escape(line)}</p>")
    return "<!doctype html><meta charset=utf-8>" + "\n".join(out)


class Handler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, fmt, *args):
        stamp = datetime.datetime.now().isoformat(timespec="seconds")
        entry = f"{stamp} {self.address_string()} {fmt % args} UA={self.headers.get('User-Agent','-')}\n"
        with ACCESS_LOG.open("a") as fh:
            fh.write(entry)
        sys.stderr.write(entry)

    def _send(self, body: str, ctype: str):
        data = body.encode()
        self.send_response(200)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        path = self.path.rstrip("/") or "/"

        if path == "/":
            rows = "".join(
                f"<li><a href='/doc/{n}'>{n}</a> — {m['context']} · {m['category']}</li>"
                for n, m in MANIFEST.items()
            )
            return self._send(
                f"<!doctype html><meta charset=utf-8><h1>Injection lab</h1><ul>{rows}</ul>",
                "text/html; charset=utf-8",
            )

        if path == "/manifest":
            return self._send(json.dumps(MANIFEST, indent=2), "application/json")

        for prefix, ctype, transform in (
            ("/doc/", "text/html; charset=utf-8", render),
            ("/raw/", "text/markdown; charset=utf-8", lambda s: s),
        ):
            if path.startswith(prefix):
                name = path[len(prefix):]
                f = CORPUS / f"{name}.md"
                if not f.is_file():
                    self.send_error(404, "no such document")
                    return
                return self._send(transform(f.read_text()), ctype)

        self.send_error(404, "unknown route")


if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("127.0.0.1", PORT), Handler) as httpd:
        print(f"injection lab on http://127.0.0.1:{PORT}  ({len(MANIFEST)} documents)", flush=True)
        httpd.serve_forever()
