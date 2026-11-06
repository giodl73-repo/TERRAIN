"""Build TERRAIN's bounded browser workbench from its native Rust core."""
import argparse
import json
import os
import shutil
import subprocess
from pathlib import Path

from input_custody import destination, digest, strict_json
from rust_build import configure, finish

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/TERRAIN"))
    parser.add_argument("--inputs",type=Path,required=True,help="New or verified original source input folder outside this checkout")
    parser.add_argument("--cargo-home",type=Path,required=True,help="New or verified owned Cargo home outside this checkout")
    parser.add_argument("--target",type=Path,default=Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'target')))
    args = parser.parse_args()
    output = destination(args.output)
    if output == ROOT / "dist" or ROOT / "dist" not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    context = configure(args.inputs,args.cargo_home,args.target)
    dependency_notices = (ROOT/'DEPENDENCY-NOTICES.txt').read_bytes()
    if dependency_notices != context['verified']['notices']:
        raise ValueError('Published original dependency notices differ from input archives')
    runtime_notice = strict_json((ROOT/'notices/RUNTIME-NOTICES.json').read_bytes())
    if runtime_notice['file'] != 'rust-1.95.0-COPYRIGHT-library.html':
        raise ValueError('Unexpected Rust runtime notice path')
    runtime_bytes = (ROOT/'notices'/runtime_notice['file']).read_bytes()
    if runtime_notice['toolchain'] != '1.95.0' or runtime_notice['file'] != 'rust-1.95.0-COPYRIGHT-library.html' or len(runtime_bytes) != runtime_notice['bytes'] or digest(runtime_bytes) != runtime_notice['sha256']:
        raise ValueError('Original Rust runtime notices differ')
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "terrain-web", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/terrain_web.wasm"
    output.mkdir(parents=True, exist_ok=False)
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "planner-session.js", "worker.js", "sample.csv"):
        shutil.copy2(ROOT / "web" / name, output / name)
    shutil.copy2(ROOT / "LICENSE", output / "LICENSE")
    (output/'DEPENDENCY-NOTICES.txt').write_bytes(dependency_notices)
    (output/'RUST-RUNTIME-NOTICES.html').write_bytes(runtime_bytes)
    (output / ".nojekyll").touch()
    finish(context)
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Planner exceeds its 5 MB delivery budget")
    print(f"Pages planner: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
