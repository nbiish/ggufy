**ggufy: Unified GGUF Model Wrapper for llama.cpp and Ollama**

<div align="center">
  <hr width="50%">
  
  <h3>Support This Project</h3>
  <div style="display: flex; justify-content: center; gap: 20px; margin: 20px 0;">
    <div>
      <h4>Stripe</h4>
      <img src="qr-stripe-donation.png" alt="Scan to donate" width="180"/>
      <p><a href="https://raw.githubusercontent.com/nbiish/license-for-all-works/8e9b73b269add9161dc04bbdd79f818c40fca14e/qr-stripe-donation.png">Donate via Stripe</a></p>
    </div>
    <div style="display: flex; align-items: center;">
      <a href="https://www.buymeacoffee.com/nbiish"><img src="https://img.buymeacoffee.com/button-api/?text=Buy me a coffee&emoji=&slug=nbiish&button_colour=FFDD00&font_colour=000000&font_family=Cookie&outline_colour=000000&coffee_colour=ffffff" /></a>
    </div>
  </div>
  
  <hr width="50%">
</div>

**Overview**
- Discovers local `.gguf` models from llama.cpp caches and Ollama library blobs and presents them as a single, easy-to-address collection.
- Unifies model naming and access via a symlink directory (`<model>-<tag>.gguf` for Ollama, original filename for llama.cpp).
- Adds smart run modes, cloud fallback, simple prompting, and quiet output improvements compared to raw Ollama and llama.cpp.

**Prerequisites**
- `llama-server` and `llama-cli` available on PATH.
- Ollama installed (`ollama` on PATH) to resolve library manifests and (optionally) run cloud models.
- macOS/Linux/Windows supported; llama.cpp cache detection adapts to OS.

**Installation**
- Homebrew (local formula): `brew install --build-from-source ./formula/ggufy.rb`
- Cargo (user install): `cargo install --path .`
- Ensure PATH includes Cargo bin: `export PATH="$HOME/.cargo/bin:$PATH"`
- Dev build: `cargo build` → binaries in `./target/debug/ggufy` and `./target/debug/ggufy-simple`

**Model Discovery**
- llama.cpp caches
  - macOS: `~/Library/Caches/llama.cpp/`
  - Linux: `~/.cache/llama.cpp/`
  - Windows: `%LOCALAPPDATA%/llama.cpp`
  - Override: `LLAMA_CPP_CACHE_DIR`
- Ollama blobs: `~/.ollama/models/blobs/sha256-*`
- Ollama library manifests: `~/.ollama/models/manifests/registry.ollama.ai/library/<model>/<tag>`

**Symlink Directory**
- Default: `~/.guffy/models`
- Override: `--link-dir <PATH>` or `GGUFY_MODELS_DIR`
- Force relink: `--force` replaces existing links
- Recommended alternative: `~/.ggufy` via `--link-dir ~/.ggufy` for concise `-m` paths

**Naming Conventions**
- Ollama models: symlinked as `<model>-<tag>.gguf` (resolved from library manifest `sha256` digests, choosing the largest valid GGUF blob)
- llama.cpp models: symlinked with their original `.gguf` filenames
- Resolution by name: `ggufy` accepts shorthand model names; if the extension is missing, `.gguf` is appended during link-dir resolution

**Key Commands**
- `ggufy list`
  - Prints local Ollama models (like `ollama list`) and all discovered llama.cpp GGUFs with clean spacing.
  - Symlinks both sets into your link directory and reports the link count.
  - Non-blocking: captures Ollama list output synchronously to avoid stalling.

- `ggufy locate <regex>`
  - Prints absolute paths from discovered locations that match the pattern.

- `ggufy link [--link-dir <PATH>] [--force]`
  - Bulk refreshes symlinks for all known models into your link directory.

- `ggufy run <model[:tag]> [EXTRA...]`
  - Smart runner that chooses between local GGUF and Ollama Cloud.
  - If the tag is `cloud` or the model is available as a Cloud model, routes to `ollama run <model>:cloud`.
  - Otherwise resolves local GGUF from the Ollama library manifest or your link directory and runs via `llama-server`.
  - To enable multimodal, pass extras through with `--` and specify `--mmproj <path>` manually.

- `ggufy simple <model or model:tag> "prompt..."`
  - One-shot prompt executor:
    - Local `.gguf`: runs `llama-cli -m <path> -p "<prompt>" -no-cnv` with stderr suppressed.
    - Cloud: runs `ollama run <model>:cloud "<prompt>"` with conversation mode preserved and native output.
  - Aligns with the symlink names produced by `list` and `link`, so `-m` paths are consistent.

**llama.cpp Commands**
- `ggufy llama hf <org/repo>`
  - Runs `llama-server -hf <org/repo>`, then symlinks the cached `.gguf` into the link directory.
- `ggufy llama serve <model-or-path> [EXTRA...]`
  - Runs `llama-server -m <path>` resolving names from your link directory.
- `ggufy llama cli <model-or-path> [EXTRA...]`
  - Runs `llama-cli -m <path>` for local generation.
- Default port: `12434` unless overridden by `--port <n>`

**Ollama Commands**
- `ggufy ollama serve <path-or-model:tag> [EXTRA...]`
  - For `<path>`: serves the blob file directly.
  - For `<model:tag>`: resolves library manifests to blob digests, picks the largest valid `.gguf`, symlinks it as `<model>-<tag>.gguf`, and serves it.
- `ggufy ollama run <model:tag> [EXTRA...]`
  - Resolves the library manifest and serves the resolved blob.
- `ggufy ollama link-all`
  - Symlinks every locally installed Ollama model into your link directory with `<model>-<tag>.gguf` naming.
- Default port: `11434` unless overridden by `--port <n>`

**Multimodal Support**
- No automatic injection. Provide `-- --mmproj <path>` when calling `serve`/`run` to pass a matching mmproj to `llama-server`.

**Output and UX Improvements**
- Clean listing: synchronous capture of `ollama list` avoids hanging output; sections are spaced for readability.
- Quiet simple mode: stderr suppressed and `-no-cnv` enabled to avoid backend noise.
- Unified naming and resolution: use consistent names across Ollama and llama.cpp without tracking blob filenames.

**Ports**
- Global override: `--port <n>` for serve/run/hf commands.
- Defaults: llama.cpp `12434`, Ollama `11434`.

**Examples**
- Aggregate and link:
  - `./target/debug/ggufy list`
  - `./target/debug/ggufy --link-dir ~/.ggufy list`
- Serve llama.cpp:
  - `./target/debug/ggufy llama serve LiquidAI_LFM2-1.2B-GGUF_LFM2-1.2B-Q4_K_M.gguf`
- Serve Ollama library:
  - `./target/debug/ggufy ollama serve qwen3:latest`
- Smart run:
  - `./target/debug/ggufy run llama3`
  - `./target/debug/ggufy run llama3 cloud`
  - `./target/debug/ggufy run qwen3 latest -- --threads 6 --ctx-size 8192`
- Simple prompt:
  - `./target/debug/ggufy simple granite4-latest "who are the anishinaabe"`
  - `./target/debug/ggufy simple llama3:cloud "who are the anishinaabe"`

**Quick Start**
- Install dependencies: `brew install ollama llama.cpp`
- Install ggufy: `cargo install --path .` and ensure `~/.cargo/bin` is on PATH
- Prepare link dir (optional): `export GGUFY_MODELS_DIR="$HOME/.ggufy"`
- List models and create links: `ggufy list`
- Run locally or cloud smartly: `ggufy run qwen3`
- One-shot prompt:
  - Local: `ggufy simple granite4-latest "question..."`
  - Cloud: `ggufy simple qwen3:cloud "question..."`

**Command Cheatsheet (ggufy vs raw)**
- List:
  - ggufy: `ggufy list`
  - ollama: `ollama list`
  - llama.cpp: N/A (ggufy discovers `~/Library/Caches/llama.cpp/*.gguf`)
- Run:
  - ggufy: `ggufy run <model[:tag]> [--link-dir <path>] [--port <n>] [EXTRA...]`
  - ollama cloud: `ollama run <model>:cloud "<prompt>"`
  - llama.cpp local: `llama-server -m <path> [--mmproj <path>]`
- Simple prompt:
  - ggufy: `ggufy simple <model or model:tag> "<prompt>"`
  - ollama cloud: `ollama run <model>:cloud "<prompt>"`
  - llama.cpp local: `llama-cli -m <path> -p "<prompt>" -no-cnv`
- Link refresh:
  - ggufy: `ggufy link`
  - ollama blobs discovered under `~/.ollama/models/blobs` and linked as `<model>-<tag>.gguf`

**Cloud Behavior**
- `:cloud` always runs via Ollama; conversation mode and native output are preserved.
- `ggufy run <model>` falls back to cloud when local GGUF is unavailable and a cloud model exists.
- `ggufy simple <model:cloud> "prompt"` uses Ollama schema `ollama run <model>:cloud "prompt"`.

**Environment Variables**
- `GGUFY_MODELS_DIR`: override the link directory (default `~/.guffy/models`)
- `LLAMA_CPP_CACHE_DIR`: override llama.cpp cache discovery

**Troubleshooting**
- `ggufy` not found: install via Cargo or Brew; ensure PATH includes appropriate bin directories.
- `llama-server` / `llama-cli` not found: install `llama.cpp` (e.g., via Homebrew).
- `ollama` not found: install via Homebrew; verify cloud connectivity separately.
- Missing local model: run `ggufy link` or use cloud via `:cloud`.

**Why ggufy Simplifies Ollama and llama.cpp**
- Unified naming and link dir: access everything with consistent `-m` paths without hunting `sha256-*` blobs.
- Smart engine selection: automatic local-or-cloud routing removes guesswork.
- Multimodal support: explicit mmproj forwarding via extras for precise control.

**License & Contributing**
- License: see `LICENSE` (synced from `license-for-all-works` authoritative source).
- Contributing: see `CONTRIBUTING.md` (synced from `license-for-all-works`).

**Policies**
- Terms of Service: https://raw.githubusercontent.com/nbiish/license-for-all-works/refs/heads/main/Terms-of-Service.md
- Privacy Policy: https://raw.githubusercontent.com/nbiish/license-for-all-works/refs/heads/main/Privacy-Policy.md

**Citation**
```bibtex
@misc{ggufy2025,
  author/creator/steward = {ᓂᐲᔥ ᐙᐸᓂᒥᑮ-ᑭᓇᐙᐸᑭᓯ (Nbiish Waabanimikii-Kinawaabakizi), also known legally as JUSTIN PAUL KENWABIKISE, professionally documented as Nbiish-Justin Paul Kenwabikise, Anishinaabek Dodem (Anishinaabe Clan): Animikii (Thunder), descendant of Chief ᑭᓇᐙᐸᑭᓯ (Kinwaabakizi) of the Beaver Island Band and enrolled member of the sovereign Grand Traverse Band of Ottawa and Chippewa Indians},
  title/description = {ggufy},
  type_of_work = {Indigenous digital creation/software incorporating traditional knowledge and cultural expressions},
  year = {2025},
  publisher/source/event = {GitHub repository under tribal sovereignty protections},
  howpublished = {\url{https://github.com/nbiish/ggufy}},
  note = {Authored and stewarded by ᓂᐲᔥ ᐙᐸᓂᒥᑮ-ᑭᓇᐙᐸᑭᓯ (Nbiish Waabanimikii-Kinawaabakizi), also known legally as JUSTIN PAUL KENWABIKISE, professionally documented as Nbiish-Justin Paul Kenwabikise, Anishinaabek Dodem (Anishinaabe Clan): Animikii (Thunder), descendant of Chief ᑭᓇᐙᐸᑭᓯ (Kinwaabakizi) of the Beaver Island Band and enrolled member of the sovereign Grand Traverse Band of Ottawa and Chippewa Indians. This work embodies Indigenous intellectual property, traditional knowledge systems (TK), traditional cultural expressions (TCEs), and associated data protected under tribal law, federal Indian law, treaty rights, Indigenous Data Sovereignty principles, and international indigenous rights frameworks including UNDRIP. All usage, benefit-sharing, and data governance are governed by the COMPREHENSIVE RESTRICTED USE LICENSE FOR INDIGENOUS CREATIONS WITH TRIBAL SOVEREIGNTY, DATA SOVEREIGNTY, AND WEALTH RECLAMATION PROTECTIONS.}
}
```
- Cleaner UX: spaced list output, quiet local prompts, native cloud conversation output, consistent flags across commands.

**Testing**
- Integration script: `bash tests/run.sh`
- Validates build, list spacing, link refresh, Ollama manifest resolution, and llama.cpp serve/cli flows.

**Why ggufy Over Raw Ollama and llama.cpp**
- One naming scheme: `<model>-<tag>.gguf` for Ollama, original filenames for llama.cpp; no need to chase `sha256-*` blobs.
- One link directory: all models in a single place (`~/.guffy/models` or `~/.ggufy`) for straightforward `-m` usage.
- Smart routing: `run` chooses Cloud vs local GGUF automatically based on availability and tag.
- Multimodal awareness: automatic `mmproj` detection and injection.
- Cleaner UX: non-blocking list, suppressed noise in simple mode, consistent flags and overrides across ecosystems.