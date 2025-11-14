**Objectives**

* Aggregate models from llama.cpp caches and Ollama library into a single symlinked directory.

* Provide a drop‑in wrapper: `guffy -c` maps to llama.cpp (`llama-server`/`llama-cli`), `guffy -o` maps to Ollama library models.

* Allow users to run llama.cpp directly from the ggufy repository’s model directory or `~/.guffy/models`.

**Discovery**

* Llama.cpp cache paths:

  * macOS: `~/Library/Caches/llama.cpp/`

  * Linux: `~/.cache/llama.cpp/`

  * Windows: `%LOCALAPPDATA%/llama.cpp/`

* Hugging Face pulls via `-hf` create cached `.gguf` in those paths.

* Ollama library manifests: `~/.ollama/models/manifests/registry.ollama.ai/library/<model>/<tag>`; blobs in `~/.ollama/models/blobs/sha256-*`.

* Detect `.gguf` by header (`GGUF`) and include multimodal `mmproj-*.gguf` files.

**Symlink Strategy**

* Default link dir: `~/.guffy/models`.

* Optional repo link dir: `./models` inside the ggufy repo (configurable via `--link-dir` or env `GGUFY_MODELS_DIR`).

* Idempotent linking: skip if link exists; refresh with `--force` to relink.

* After `-hf` run, scan cache for matching owner/repo and link latest modified `.gguf`.

* For Ollama, parse manifests to resolve digests; link the largest GGUF blob as the primary model plus any `mmproj` GGUF if present.

**Run Strategy**

* llama.cpp mode (`-c`):

  * `guffy -c hf <org/repo>` → spawn `llama-server -hf <org/repo>`; on download completion, link cached `.gguf` into link dir.

  * `guffy -c serve -m <model.gguf>` → spawn `llama-server -m <link-dir>/<model.gguf>`; pass `--mmproj` automatically if `mmproj-*.gguf` exists and the model requires it.

  * `guffy -c cli -m <model.gguf> [args...]` → run `llama-cli` for local generation.

* Ollama mode (`-o`):

  * `guffy -o run <model>:<tag>` → resolve library manifest, pick GGUF blob, link `<model>-<tag>.gguf`, spawn `llama-server -m <blob>`.

  * Restrict to library namespace; do not resolve non‑library manifests.

**CLI Surface**

* `guffy list` → scan llama.cpp caches + Ollama blobs, link all `.gguf` + `mmproj-*.gguf` into link dir; print count.

* `guffy locate <pattern>` → show resolved absolute paths (cache or blob) and link status.

* `guffy link [--link-dir <path>] [--force]` → bulk refresh links.

* `guffy -c hf <org/repo>` / `guffy -c serve -m <name>` / `guffy -c cli -m <name>`.

* `guffy -o run <model>:<tag>`.

**Path Resolution & Ports**

* Port configuration: `--port <n>` forwarded to `llama-server`.

* Binary resolution: require `llama-server`/`llama-cli` on PATH; error with actionable message if missing.

* Cross‑platform path detection (macOS/Linux/Windows) using OS APIs; fall back to env overrides.

**Validation Plan**

* Build: `cargo build` must succeed; binary at `./target/debug/ggufy`.

* `list`: creates symlinks for discovered GGUFs; re-run to confirm idempotency.

* `-c hf LiquidAI/LFM2-1.2B-GGUF`: starts server; cache file appears; symlink created.

* `-o run <model>:<tag>`: manifest read; GGUF blob validated; symlink created; server launched.

* Check multimodal: ensure `mmproj-*.gguf` is detected and passed to server.

* Negative tests: missing manifests, no GGUF; produce clear errors.

**Edge Cases**

* Multiple GGUF candidates: choose largest file; allow `--select <name>` override.

* Incomplete downloads: skip `.downloadInProgress`; retry on completion.

* Broken links: validate target exists; repair with `--force` relink.

**Security & UX**

* No secrets logged; paths printed succinctly.

* Dry‑run flag to preview actions without spawning servers.

**Next Steps**

* Implement `--link-dir` and `--force`, multimodal `--mmproj` detection, and `cli`/`serve` variants.

* Execute validation commands, capture logs, and iterate on edge cases.

* Document supported platforms and known limitations in the repo.

