import os
import sys
import shutil
import subprocess

def main() -> None:
    exe = shutil.which("ggufy")
    if not exe:
        print("ggufy binary not found on PATH. Install via brew or cargo.")
        sys.exit(127)
    subprocess.run([exe, *sys.argv[1:]], check=False)