## Codebase Review
- Audit CLI surface: confirm nested subcommands `ollama` (serve, run) and `llama` (hf, serve, cli) operate with correct mode gating and parameter parsing.
- Validate defaults: ensure `--port` fallback is `11434` for Ollama flows and `12434` for llama.cpp flows.
- Confirm model discovery: check path scanning of `~/.ollama/models/blobs`, `~/.ollama/models/manifests/registry.ollama.ai/library`, and OS-specific llama.cpp caches (macOS, Linux, Windows), plus `LLAMA_CPP_CACHE_DIR` override.
- Verify symlink strategy: idempotent linking into `~/.guffy/models` or `GGUFY_MODELS_DIR`, with `--force` replacement and multimodal `mmproj` detection.
- Review test coverage: ensure `tests/run.sh` exercises build, list, locate, and serve paths; plan additional cases for hf, cli, run, link, `--force`, `--port`, and `--link-dir`.

## README Verification
- Check that documentation clearly explains:
- Command separation: `ggufy ollama ...` vs `ggufy llama ...` with their defaults and behaviors.
- Model discovery: typical filesystem locations for Ollama blobs/manifests and llama.cpp caches.
- Ports: default serve ports and `--port` override.
- Symlinks: how `list`, `link`, and `--force` work; where links are stored; how to set `--link-dir`/`GGUFY_MODELS_DIR`.
- Multimodal support: automatic `mmproj-*.gguf` detection and passing to `llama-server`.
- External prerequisites: `llama-server`/`llama-cli` availability on PATH; Ollama installed for library manifests.

## GGUF Usage Coverage
- Add a concise section that walks users through:
- Aggregation: run `ggufy list` to collect `.gguf` from caches/blobs.
- Serving locally: `ggufy llama serve <model-or-path>` (default 12434); multimodal note.
- Serving Ollama library or blob: `ggufy ollama serve <model:tag|path>` (default 11434).
- Hugging Face pull: `ggufy llama hf <org/repo>` and symlink outcome.
- Locating and linking: `ggufy locate <regex>` and `ggufy link [--link-dir <path>] [--force]`.

## Accuracy & Currency
- Ensure paths and defaults match current implementation and platform specifics, with macOS/Linux/Windows cache resolution.
- Keep README strictly accurate: no extraneous content, no changes to existing structure beyond requested coverage; update existing sections rather than adding commentary.

## Deliverables
- A reviewed codebase report covering commands, flags, defaults, symlink logic, path scanning, and test script scope.
- An updated README (or existing doc section) that is accurate, current, and comprehensive for `.gguf` usage without altering the document’s existing structural style.

## Next Steps
- Upon approval, expand test coverage for missing cases (hf, cli, run, link, overrides) and update README accordingly, preserving original structure while improving clarity on `.gguf` usage.