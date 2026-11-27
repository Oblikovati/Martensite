#!/usr/bin/env python3
"""Build and run the Martensite multi-threaded WASM + WebGPU E2E harness in headless Chromium."""
import functools
import gzip
import http.server
import json
import os
import pathlib
import subprocess
import sys
import threading

ROOT = pathlib.Path(__file__).resolve().parent
REPO = ROOT.parent.parent
PKG = ROOT / "pkg"
WASM_TARGET = "wasm32-unknown-unknown"
NIGHTLY = "nightly-2026-09-25"
RUSTFLAGS = (
    "-C target-feature=+atomics,+bulk-memory,+mutable-globals "
    "-C link-arg=--shared-memory "
    "-C link-arg=--max-memory=1073741824 "
    "-C link-arg=--import-memory "
    "-C link-arg=--export=__wasm_init_tls "
    "-C link-arg=--export=__tls_size "
    "-C link-arg=--export=__tls_align "
    "-C link-arg=--export=__tls_base"
)


class CoopCoepHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        super().end_headers()

    def log_message(self, format, *args):
        # Intentionally silent so stderr only reports E2E progress and failures.
        return


def build_wasm():
    env = os.environ.copy()
    env["RUSTFLAGS"] = RUSTFLAGS
    cmd = [
        "cargo",
        f"+{NIGHTLY}",
        "build",
        "--manifest-path",
        str(ROOT / "Cargo.toml"),
        "--release",
        "--target",
        WASM_TARGET,
        "-Z",
        "build-std=panic_abort,std",
    ]
    print("Building wasm32 +atomics release E2E module...", file=sys.stderr)
    subprocess.run(cmd, env=env, check=True)
    wasm_path = ROOT / "target" / WASM_TARGET / "release" / "martensite_wasm_e2e.wasm"
    PKG.mkdir(exist_ok=True)
    subprocess.run(
        [
            "wasm-bindgen",
            "--target",
            "web",
            "--out-dir",
            str(PKG),
            str(wasm_path),
        ],
        check=True,
    )
    out_wasm = PKG / "martensite_wasm_e2e_bg.wasm"
    raw_bytes = out_wasm.read_bytes()
    gz_bytes = gzip.compress(raw_bytes, compresslevel=9)
    br_size = None
    try:
        import brotli
        br_size = len(brotli.compress(raw_bytes))
    except ImportError:
        pass
    return {
        "wasmRawBytes": len(raw_bytes),
        "wasmGzipBytes": len(gz_bytes),
        "wasmBrotliBytes": br_size,
    }


def run_chromium(size_stats):
    from playwright.sync_api import sync_playwright

    handler = functools.partial(CoopCoepHandler, directory=str(ROOT))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    port = server.server_address[1]
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    console_logs = []
    try:
        with sync_playwright() as p:
            browser = p.chromium.launch(
                headless=True,
                args=[
                    "--enable-unsafe-webgpu",
                    "--use-angle=vulkan",
                    "--enable-features=Vulkan",
                    "--ignore-gpu-blocklist",
                    "--no-sandbox",
                ],
            )
            version = browser.version
            page = browser.new_page()
            page.on("console", lambda msg: console_logs.append(f"[page:{msg.type}] {msg.text}"))
            page.on("pageerror", lambda err: console_logs.append(f"[pageerror] {err}"))
            page.on(
                "worker",
                lambda w: w.on(
                    "console",
                    lambda msg: console_logs.append(f"[worker:{msg.type}] {msg.text}"),
                ),
            )
            page.goto(f"http://127.0.0.1:{port}/index.html")
            page.wait_for_function(
                "window.__e2eResult !== null || window.__e2eError !== null",
                timeout=120_000,
            )
            error = page.evaluate("window.__e2eError")
            result = page.evaluate("window.__e2eResult")
            browser.close()
    finally:
        server.shutdown()

    if error:
        print("Console logs:\n" + "\n".join(console_logs), file=sys.stderr)
        raise RuntimeError(f"Chromium E2E failed: {error}")

    report = {"chromiumVersion": version, **size_stats, **result}
    print(json.dumps(report, indent=2))
    assert report["crossOriginIsolated"] is True
    assert report["mainThreadBlocked"] is False
    assert report["activeWorkers"] >= 1
    assert report["validationErrors"] == 0, f"WebGPU validation errors: {report}"
    assert report["pbrLitPixels"] > 500, f"Too few PBR pixels: {report}"
    assert report["rtLitPixels"] > 300, f"Too few RT pixels: {report}"


if __name__ == "__main__":
    if "--no-build" in sys.argv:
        out_wasm = PKG / "martensite_wasm_e2e_bg.wasm"
        raw_bytes = out_wasm.read_bytes()
        stats = {
            "wasmRawBytes": len(raw_bytes),
            "wasmGzipBytes": len(gzip.compress(raw_bytes, compresslevel=9)),
            "wasmBrotliBytes": None,
        }
    else:
        stats = build_wasm()
    run_chromium(stats)
