**ggufy: Unified GGUF Model Wrapper**

![CI](https://github.com/nbiish/ggufy/actions/workflows/ci.yml/badge.svg)

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

**ggufy** unifies your local GGUF models from llama.cpp and Ollama into a single, easy-to-use collection. It handles symlinking, smart execution (local vs cloud), and provides a cleaner CLI experience.

### Features
- **Unified Discovery**: Finds models in `~/Library/Caches/llama.cpp` and `~/.ollama/models/blobs`.
- **Smart Symlinking**: Creates a consistent `~/.guffy/models` directory (or custom path).
- **Intelligent Runner**: Automatically chooses between local GGUF and Ollama Cloud.
- **Audio/TTS Support**: Detects audio models by name (containing "audio") and routes them to `llama-cli`.
- **Clean UX**: Non-blocking lists, quiet output, and consistent flags.

### Installation

Install via **Cargo** (Rust) or **pip** (Python, installs pre-built Rust binaries):

```bash
# From crates.io
cargo install ggufy

# From PyPI (downloads Rust binary via maturin)
pip install ggufy

# From source
git clone https://github.com/nbiish/ggufy.git
cd ggufy
cargo build --release
# Binaries will be in target/release/
```

After installation, ensure the binaries are on your PATH:
```bash
# For cargo installations
export PATH="$HOME/.cargo/bin:$PATH"

# For pip installations (usually automatic)
which ggufy
```

### Quick Start
1.  **List & Link**: `ggufy list` (Refreshes links in `~/.guffy/models`)
2.  **Run Smart**: `ggufy run <model>` (Uses local if available, else cloud)
3.  **Simple Prompt**: `ggufy simple <model> "prompt"`

### Commands
| Command | Description |
| :--- | :--- |
| `ggufy list` | Lists all local models and refreshes symlinks. |
| `ggufy link` | Refreshes symlinks without listing. |
| `ggufy locate <regex>` | Finds absolute paths of models matching pattern. |
| `ggufy run <model> [args]` | Runs model via `llama-server` or `ollama run`. |
| `ggufy simple <model> "txt"` | One-shot prompt. Quiet mode for local, chat mode for cloud. |
| `ggufy llama <cmd>` | Wrappers for `llama-server` and `llama-cli`. |
| `ggufy ollama <cmd>` | Wrappers for `ollama serve/run`. |
| `ggufy -o <args>` | Passthrough to `ollama <args>`. |
| `ggufy -c <args>` | Passthrough to `llama-*` tools (e.g. `ggufy -c server ...`). |

### Configuration
- **Link Directory**: `export GGUFY_MODELS_DIR="$HOME/.ggufy"` (or use `--link-dir`)
- **Cache Override**: `export LLAMA_CPP_CACHE_DIR="..."`
- **Ports**: Default `12434` (llama), `11434` (ollama). Override with `--port`.

### Audio/Multimodal
- **Audio Models**: If the model name contains "audio", `ggufy` automatically uses `llama-cli` for execution.
- **Multimodal**: Pass projection files manually: `ggufy run model -- --mmproj path/to/mmproj`.

### Directory Structure
```
.
├── .github
│   └── workflows
├── docs
│   └── llama_cpp_overview.md
├── src
│   ├── bin
│   └── main.rs
├── tests
│   ├── integration.rs
│   └── run.sh
├── Cargo.toml
├── pyproject.toml
├── README.md
└── LICENSE
```

### License & Citation
See `LICENSE` and `CONTRIBUTING.md`.

### Policies
- **Terms of Service**: <https://raw.githubusercontent.com/nbiish/license-for-all-works/refs/heads/main/Terms-of-Service.md>
- **Privacy Policy**: <https://raw.githubusercontent.com/nbiish/license-for-all-works/refs/heads/main/Privacy-Policy.md>

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

### Copyright
Copyright © 2025 ᓂᐲᔥ ᐙᐸᓂᒥᑮ-ᑭᓇᐙᐸᑭᓯ (Nbiish Waabanimikii-Kinawaabakizi), also known legally as JUSTIN PAUL KENWABIKISE, professionally documented as Nbiish-Justin Paul Kenwabikise, Anishinaabek Dodem (Anishinaabe Clan): Animikii (Thunder), a descendant of Chief ᑭᓇᐙᐸᑭᓯ (Kinwaabakizi) of the Beaver Island Band, and an enrolled member of the sovereign Grand Traverse Band of Ottawa and Chippewa Indians. This work embodies Traditional Knowledge and Traditional Cultural Expressions. All rights reserved.