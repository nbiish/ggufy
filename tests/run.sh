#!/usr/bin/env bash
set -euo pipefail

echo "[BUILD] ggufy"
cargo build

echo "[LIST] aggregate and link models"
./target/debug/ggufy list

echo "[LOCATE] sample discovered models"
./target/debug/ggufy locate ".*" | head -n 20 || true

echo "[SERVE-LLAMA] launch llama.cpp server on default port"
LLAMA_MODEL=$(ls -1 "$HOME/.guffy/models" | grep -v '^sha256-' | head -n 1)
if [[ -n "${LLAMA_MODEL:-}" ]]; then
  ./target/debug/ggufy llama serve "$LLAMA_MODEL"
fi

echo "[SERVE-OLLAMA] launch via ollama model blob on default port"
OLLAMA_MODEL=$(ls -1 "$HOME/.guffy/models" | grep '^sha256-' | head -n 1)
if [[ -n "${OLLAMA_MODEL:-}" ]]; then
  ./target/debug/ggufy ollama serve "$OLLAMA_MODEL"
fi

echo "[HF] launch llama.cpp with Hugging Face on explicit port"
./target/debug/ggufy --port 8081 llama hf LiquidAI/LFM2-1.2B-GGUF || true

echo "[CLI] run llama-cli for a local model if available"
if [[ -n "${LLAMA_MODEL:-}" ]]; then
  ./target/debug/ggufy -c cli "$LLAMA_MODEL" || true
fi

echo "[OLLAMA-RUN] resolve library model if present"
if ollama list | awk '{print $1}' | grep -q '^qwen3:latest$'; then
  ./target/debug/ggufy ollama run qwen3:latest || true
fi

echo "[LINK-DIR] relink into repo-local models directory"
mkdir -p models
./target/debug/ggufy --link-dir ./models --force link

echo "[OVERRIDES] validate port override for llama serve"
if [[ -n "${LLAMA_MODEL:-}" ]]; then
  ./target/debug/ggufy --port 9000 llama serve "$LLAMA_MODEL"
fi

echo "[OLLAMA LIST] show installed models"
ollama list || true

echo "[LLAMA.CPP HOME] inspect home and cache directories"
ls -la "$HOME/llama.cpp" | head -n 20 || true
ls -la "$HOME/Library/Caches/llama.cpp" | head -n 20 || true

echo "[DONE] integration script completed"