**Overview**
- `llama.cpp` enables local LLM inference with minimal setup across CPU and GPU backends.
- Apple Silicon is optimized via Accelerate and Metal; x86 uses AVX/AVX2/AVX512/AMX; NVIDIA via CUDA; AMD via HIP; Vulkan and SYCL supported.
- Supports quantized integer formats from 1.5-bit to 8-bit for speed and memory efficiency.
- Works with many model families (LLaMA 1–3, Mistral/Mixtral, Gemma, Qwen, Phi, Yi, DeepSeek, DBRX, OLMo, etc.).

**Key Binaries**
- `llama-server`: OpenAI-compatible HTTP server for running models.
- `llama-cli`: CLI for local generation; can run via local `.gguf` or directly from Hugging Face.

**Essential Commands**
- `llama-cli -m <path/to/model.gguf>` runs a local GGUF model.
- `llama-cli -hf <org/model-GGUF>` downloads and runs a model from Hugging Face.
- `llama-server -hf <org/model-GGUF>` launches an API server and pulls the model automatically.

**Model Storage Paths**
- macOS cache: `~/Library/Caches/llama.cpp/` stores downloaded GGUF files when using `-hf`.
- Example log shows a download path like `~/Library/Caches/llama.cpp/<owner>_<repo>_<filename>.gguf`.
- Ollama stores blobs in `~/.ollama/models/blobs/` and manifests under `~/.ollama/models/manifests/registry.ollama.ai/library/`.

**GGUF Format**
- GGUF is the standard optimized format used by `llama.cpp`.
- Many Hugging Face repos publish `.gguf` files directly; Ollama library models often include GGUF blobs.

**Using Hugging Face**
- `-hf` accepts `org/repo` for GGUF repos and automates download to the local cache.
- Works for both `llama-cli` and `llama-server` to simplify first-run setup.

**Ollama Integration Notes**
- Ollama frequently uses GGUF under the hood and stores model blobs by digest.
- Library models live under `registry.ollama.ai/library`; manifests list layer digests.
- GGUF blobs can be detected by reading the file header (`GGUF`) and run via `llama-server -m <blob>`.

**Wrap Strategy in ggufy**
- For `-c` mode: call `llama-server -hf <org/model-GGUF>` and then symlink the cached `.gguf` into `~/.guffy/models/`.
- For `-o` mode: verify the model exists under Ollama library manifests, locate GGUF blob in `~/.ollama/models/blobs/`, symlink into `~/.guffy/models/`, then run `llama-server -m <blob>`.
- `guffy list` scans `~/Library/Caches/llama.cpp` and `~/.ollama/models/blobs` for `.gguf` and ensures symlinks in `~/.guffy/models/`.

**References**
- Quick start, supported models, and binary usage are covered in the `llama.cpp` README.
- Notes: Ollama uses `llama.cpp` internally and stores GGUF blobs addressed by `sha256` digests.