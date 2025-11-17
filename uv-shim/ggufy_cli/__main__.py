import os
import sys
import shutil
import subprocess
import platform
import tempfile
import urllib.request
import tarfile
import zipfile
from pathlib import Path

REPO = "nbiish/ggufy"


def release_assets_url(tag: str) -> str:
    return f"https://github.com/{REPO}/releases/download/{tag}/"


def detect_platform() -> tuple[str, str]:
    sys_os = platform.system().lower()
    arch = platform.machine().lower()
    if sys_os == "darwin":
        target = "aarch64-apple-darwin" if arch in {"arm64", "aarch64"} else "x86_64-apple-darwin"
        return (sys_os, target)
    if sys_os == "linux":
        return (sys_os, "linux-x86_64")
    if sys_os == "windows":
        return (sys_os, "windows-x86_64")
    raise RuntimeError(f"unsupported platform: {sys_os}/{arch}")


def ensure_local_binary() -> Path:
    # Always use our cached version to avoid conflicts
    sys_os, target = detect_platform()
    # tag can be overridden by env; default to latest version string matching pyproject
    tag = os.environ.get("GGUFY_TAG", f"v0.1.2")
    base = release_assets_url(tag)
    if sys_os == "windows":
        artifact = f"ggufy-{tag}-windows-x86_64.zip"
    elif sys_os == "linux":
        artifact = f"ggufy-{tag}-linux-x86_64.tar.gz"
    else:
        artifact = f"ggufy-{tag}-{'aarch64-apple-darwin' if 'aarch64' in target else 'x86_64-apple-darwin'}.tar.gz"
    url = base + artifact
    cache_dir = Path(os.path.expanduser("~/.cache/ggufy"))
    cache_dir.mkdir(parents=True, exist_ok=True)
    dl_path = cache_dir / artifact
    if not dl_path.exists():
        try:
            urllib.request.urlretrieve(url, dl_path)
        except Exception as e:
            raise RuntimeError(f"failed to download {url}: {e}")
    out_path = cache_dir / ("ggufy.exe" if sys_os == "windows" else "ggufy")
    if not out_path.exists():
        if dl_path.suffix == ".zip":
            with zipfile.ZipFile(dl_path, "r") as zf:
                # artifact contains ggufy.exe
                for name in zf.namelist():
                    if name.endswith("ggufy.exe"):
                        zf.extract(name, cache_dir)
                        src = cache_dir / name
                        src.rename(out_path)
                        break
        else:
            with tarfile.open(dl_path, "r:gz") as tf:
                # artifact contains ggufy or ggufy-linux-x86_64
                candidates = [m for m in tf.getmembers() if os.path.basename(m.name) in {"ggufy", "ggufy-linux-x86_64"}]
                if not candidates:
                    raise RuntimeError("binary not found in archive")
                m = candidates[0]
                tf.extract(m, cache_dir)
                src = cache_dir / m.name
                src.rename(out_path)
        if sys_os != "windows":
            out_path.chmod(0o755)
    return out_path


def main() -> None:
    try:
        exe_path = ensure_local_binary()
    except Exception as e:
        print(f"failed to prepare ggufy binary: {e}")
        sys.exit(127)
    # Use exec to replace the current process, avoiding resource issues
    os.execv(str(exe_path), [str(exe_path)] + sys.argv[1:])


def main_simple() -> None:
    try:
        exe_path = ensure_local_binary()
    except Exception as e:
        print(f"failed to prepare ggufy binary: {e}")
        sys.exit(127)
    # Use exec to replace the current process, avoiding resource issues
    os.execv(str(exe_path), [str(exe_path), "simple"] + sys.argv[1:])