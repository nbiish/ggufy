**ggufy: Unified GGUF Model Wrapper**

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
```bash
# Homebrew
brew install --build-from-source ./formula/ggufy.rb

# Cargo
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"
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

### Configuration
- **Link Directory**: `export GGUFY_MODELS_DIR="$HOME/.ggufy"` (or use `--link-dir`)
- **Cache Override**: `export LLAMA_CPP_CACHE_DIR="..."`
- **Ports**: Default `12434` (llama), `11434` (ollama). Override with `--port`.

### Audio/Multimodal
- **Audio Models**: If the model name contains "audio", `ggufy` automatically uses `llama-cli` for execution.
- **Multimodal**: Pass projection files manually: `ggufy run model -- --mmproj path/to/mmproj`.

### License & Citation
See `LICENSE` and `CONTRIBUTING.md`.

```bibtex
@misc{ggufy2025,
  author = {ᓂᐲᔥ ᐙᐸᓂᒥᑮ-ᑭᓇᐙᐸᑭᓯ (Nbiish Waabanimikii-Kinawaabakizi)},
  title = {ggufy},
  year = {2025},
  url = {https://github.com/nbiish/ggufy}
}
```