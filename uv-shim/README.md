# ggufy Python Shim

A Python shim that automatically downloads and manages the appropriate `ggufy` binary for your platform.

**Website**: [ggufy.com](https://ggufy.com)  
**Repository**: [nbiish/ggufy](https://github.com/nbiish/ggufy)

## Installation

```bash
pip install ggufy
# or
uv tool install ggufy
```

## Features

- **Automatic Platform Detection**: Downloads the correct binary for your OS and architecture
- **Self-Updating**: Automatically manages binary versions
- **Dual Binaries**: Provides both `ggufy` and `ggufy-simple` commands
- **TTS/Audio Support**: Automatically detects TTS/audio prompts and routes through `ollama run`
- **GGUF Management**: Simplified GGUF model management and interaction

## Usage

After installation, you can use the tools directly:

```bash
# Basic usage
ggufy simple llama3 "What is Rust?"

# TTS/audio prompts are automatically detected
ggufy simple llama3 "generate tts audio for hello world"

# Use simplified version
ggufy-simple llama3 "Explain quantum computing"
```

## How It Works

This package acts as a shim that:
1. Detects your platform (macOS/Linux/Windows, ARM64/x86_64)
2. Downloads the appropriate `ggufy` binary from GitHub releases
3. Caches the binary locally in `~/.cache/ggufy`
4. Executes the binary with your arguments

## About ggufy

ggufy is a comprehensive tool for managing GGUF (GPT-Generated Unified Format) models, providing:
- Seamless integration with llama.cpp and Ollama
- Model serving and inference capabilities
- GGUF format conversion and management
- Cross-platform compatibility

For more information about the main ggufy project, visit [ggufy.com](https://ggufy.com) or check out the [GitHub repository](https://github.com/nbiish/ggufy).

## Environment Variables

- `GGUFY_TAG`: Override the GitHub release tag to download (default: latest version)
- `GGUFY_MODELS_DIR`: Custom directory for local models

## Requirements

- Python 3.8+
- Internet connection for initial binary download

## License

Same as the main ggufy project.