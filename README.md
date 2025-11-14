**ggufy: Unified GGUF Model Wrapper for llama.cpp and Ollama**

**Overview**
- Aggregates `.gguf` models from llama.cpp caches and Ollama blobs into a single symlink directory.
- Provides separate command spaces for Ollama (`ggufy ollama ...`) and llama.cpp (`ggufy llama ...`).
- Runs models through `llama-server` or `llama-cli` while respecting each ecosystem’s defaults.

**Prerequisites**
- `llama-server` and `llama-cli` available on PATH.
- Ollama installed for library manifests and blobs.
- macOS/Linux/Windows supported; llama.cpp cache detection adapts to OS.

**Installation**
- Build the CLI: `cargo build`
- Binary location: `./target/debug/ggufy`

**Model Discovery**
- Llama.cpp caches:
  - macOS: `~/Library/Caches/llama.cpp/`
  - Linux: `~/.cache/llama.cpp/`
  - Windows: `%LOCALAPPDATA%/llama.cpp`
  - Override: `LLAMA_CPP_CACHE_DIR`
- Ollama blobs: `~/.ollama/models/blobs/sha256-*`
- Ollama library manifests: `~/.ollama/models/manifests/registry.ollama.ai/library/<model>/<tag>`

**Symlink Directory**
- Default link directory: `~/.guffy/models`
- Override: `--link-dir <PATH>` or `GGUFY_MODELS_DIR`
- Force relink: `--force` to replace existing links

**Commands**
- `ggufy list`
  - Scans llama.cpp caches and Ollama blobs, symlinks any `.gguf` into the link directory.
- `ggufy locate <regex>`
  - Prints absolute paths that match the pattern from discovered locations.
- `ggufy link [--link-dir <PATH>] [--force]`
  - Bulk refreshes symlinks from all known sources.

**llama.cpp Commands**
- `ggufy llama hf <org/repo>`
  - Runs `llama-server -hf <org/repo>`, then symlinks the cached `.gguf` into the link directory.
- `ggufy llama serve <model-or-path>`
  - Runs `llama-server -m <path>` using the link directory resolution.
- `ggufy llama cli <model-or-path>`
  - Runs `llama-cli -m <path>` for local generation.
- Default port: `12434` unless overridden by `--port <n>`

**Ollama Commands**
- `ggufy ollama serve <path-or-model:tag>`
  - For `<path>`: serves the blob file directly.
  - For `<model:tag>`: resolves library manifests to blob digests, picks the largest valid `.gguf`, symlinks it as `<model>-<tag>.gguf`, and serves it.
- `ggufy ollama run <model:tag>`
  - Resolves library manifest and serves the resolved blob.
- Default port: `11434` unless overridden by `--port <n>`

**GGUF Usage Workflow**
- Aggregate models: `ggufy list`
- Inspect discovered models: `ggufy locate "<pattern>"`
- Serve llama.cpp model: `ggufy llama serve <model-or-path>`
- Serve Ollama library model or blob: `ggufy ollama serve <model:tag|path>`
- Pull from Hugging Face and serve: `ggufy llama hf <org/repo>`

**Multimodal Support**
- Auto-detects `mmproj-*.gguf` in the model directory or link directory and passes `--mmproj` to `llama-server` when present.

**Ports**
- Global override: `--port <n>` applies to all serve/hf commands.
- Defaults: llama.cpp `12434`, Ollama `11434`.

**Examples**
- Aggregate: `./target/debug/ggufy list`
- Serve llama.cpp: `./target/debug/ggufy llama serve mradermacher_VibeThinker-1.5B-GGUF_VibeThinker-1.5B.Q8_0.gguf`
- Serve Ollama library: `./target/debug/ggufy ollama serve qwen3:latest`
- Serve with port: `./target/debug/ggufy --port 9000 llama hf LiquidAI/LFM2-1.2B-GGUF`

**Testing**
- Run integration script: `bash tests/run.sh`
- Validates build, list, locate, and basic serve workflows for both ecosystems.