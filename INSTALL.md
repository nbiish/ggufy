# Installation Guide

This guide covers all the ways to install and use ggufy, a unified GGUF wrapper for llama.cpp and Ollama.

## Quick Install Options

### Option 1: Install from crates.io (Rust)
```bash
cargo install ggufy
```

### Option 2: Install from PyPI (Python)
```bash
pip install ggufy
# or
uv tool install ggufy
```

### Option 3: Install from GitHub (Latest)
```bash
# Clone the repository
git clone https://github.com/nbiish/ggufy.git
cd ggufy

# Install with cargo
cargo install --path .

# OR install with uv
uv tool install ./uv-shim
```

## Installation Methods

### Cargo Install (Recommended for Rust users)
```bash
cargo install ggufy
```

This installs two binaries:
- `ggufy` - Main CLI with all features
- `ggufy-simple` - Simplified version for basic usage

### UV Tool Install (Recommended for Python users)
```bash
uv tool install ggufy
```

This installs the Python shim that automatically downloads and manages the appropriate binary for your platform.

### Pip Install
```bash
pip install ggufy
```

Same as uv tool install, but uses pip instead.

### From Source
```bash
git clone https://github.com/nbiish/ggufy.git
cd ggufy
cargo build --release
sudo cp target/release/ggufy* /usr/local/bin/
```

## Usage Examples

### Basic Usage
```bash
# Use with local GGUF models
ggufy simple my-model "What is Rust?"

# Use with Ollama models
ggufy simple llama3 "Explain quantum computing"

# Use with specific model versions
ggufy simple llama3:8b "Write a poem about coding"
```

### TTS/Audio Special Handling
The tool automatically detects TTS or audio-related prompts and routes them through `ollama run`:

```bash
# These will automatically use ollama run
ggufy simple llama3 "generate tts audio for hello world"
ggufy simple llama3 "create audio output for welcome message"
```

### Advanced Usage
```bash
# List available models
ggufy list

# Run with specific options
ggufy run llama3:8b --temperature 0.7 "Tell me a story"

# Serve models
ggufy serve llama3:8b --port 8080
```

## Requirements

### For Cargo Install
- Rust toolchain (cargo)
- Optional: Ollama for cloud model support
- Optional: llama.cpp for local GGUF support

### For UV/Pip Install
- Python 3.8+
- uv or pip
- Internet connection for initial binary download

### For Full Functionality
- Ollama installed and running (for cloud models)
- llama.cpp built and available in PATH (for local GGUF models)

## Platform Support

The tool supports:
- macOS (Apple Silicon & Intel)
- Linux (x86_64)
- Windows (x86_64)

## Troubleshooting

### Common Issues

1. **Model not found**: Ensure Ollama is running and the model is available
2. **Binary not found**: Check that the tool is properly installed and in PATH
3. **Permission errors**: Ensure proper permissions for model directories

### Environment Variables

- `GGUFY_MODELS_DIR`: Custom directory for local models
- `GGUFY_TAG`: Override GitHub release tag for uv tool installs

## Verification

After installation, verify it works:
```bash
# Check version
ggufy --version
ggufy-simple --version

# Test basic functionality
ggufy simple --help
```

## Updates

### Cargo
```bash
cargo install ggufy --force
```

### UV/Pip
```bash
uv tool upgrade ggufy
# or
pip install --upgrade ggufy
```

## Support

For issues and feature requests, please visit: https://github.com/nbiish/ggufy/issues