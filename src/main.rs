use clap::{ArgAction, Parser, Subcommand};
use dirs::home_dir;
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::io::{Read, Result as IoResult};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(
    name = "ggufy",
    version,
    about = "Wraps llama.cpp and Ollama for GGUF management"
)]
struct Cli {
    #[arg(short = 'o', long = "ollama", default_value_t = false)]
    ollama: bool,
    #[arg(short = 'c', long = "llamacpp", default_value_t = false)]
    llamacpp: bool,
    #[arg(long = "link-dir", env = "GGUFY_MODELS_DIR")]
    link_dir: Option<PathBuf>,
    #[arg(long = "force", action = ArgAction::SetTrue)]
    force: bool,
    #[arg(long = "port")]
    port: Option<u16>,
    #[arg(long = "dry-run", default_value_t = false)]
    dry_run: bool,
    #[arg(long = "verbose", default_value_t = false)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hf {
        repo: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    List,
    OllamaRun {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Simple {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Run {
        target: String,
        tag: Option<String>,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Serve {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Cli {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Locate {
        pattern: String,
    },
    Link,
    OllamaServe {
        target: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Ollama {
        #[command(subcommand)]
        sub: OllamaCmd,
    },
    Llama {
        #[command(subcommand)]
        sub: LlamaCmd,
    },
}

#[derive(Subcommand)]
enum OllamaCmd {
    Serve {
        target: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Run {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    LinkAll,
}

#[derive(Subcommand)]
enum LlamaCmd {
    Hf {
        repo: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Serve {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
    Cli {
        model: String,
        #[arg(trailing_var_arg = true)]
        extra: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Hf { repo, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.llamacpp || cli.ollama {
                eprintln!("use -c for llama.cpp mode");
                std::process::exit(2);
            }
            let port = port_or_default(false, cli.port);
            run_llama_server_hf(&repo, Some(port), Some(&extra), cli.dry_run, cli.verbose);
            if let Some(p) = find_hf_cached_gguf(&repo) {
                symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force, cli.verbose).ok();
            }
        }
        Commands::Run { target, tag, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            let (name, tag_final) = normalize_model_tag(&target, tag.as_deref());
            let tag_explicit = tag.is_some();
            if tag_final.eq_ignore_ascii_case("cloud") {
                run_ollama_run(&name, &tag_final, Some(&extra), cli.dry_run, cli.verbose);
            } else if !tag_explicit {
                if let Some(blob) = resolve_ollama_library_gguf(&name, &tag_final) {
                    symlink_named_into_guffy(
                        &blob,
                        &format!("{}-{}.gguf", name, tag_final),
                        cli.link_dir.as_ref(),
                        cli.force,
                        cli.verbose,
                    )
                    .ok();
                    let port = port_or_default(false, cli.port);
                    run_llama_server_model(
                        &blob,
                        Some(port),
                        Some(&extra),
                        cli.dry_run,
                        cli.verbose,
                    );
                } else {
                    run_ollama_run(&name, "cloud", Some(&extra), cli.dry_run, cli.verbose);
                }
            } else if is_cloud_model_available(&name) {
                run_ollama_run(&name, "cloud", Some(&extra), cli.dry_run, cli.verbose);
            } else if let Some(blob) = resolve_ollama_library_gguf(&name, &tag_final) {
                symlink_named_into_guffy(
                    &blob,
                    &format!("{}-{}.gguf", name, tag_final),
                    cli.link_dir.as_ref(),
                    cli.force,
                    cli.verbose,
                )
                .ok();
                let port = port_or_default(false, cli.port);
                run_llama_server_model(&blob, Some(port), Some(&extra), cli.dry_run, cli.verbose);
            } else {
                eprintln!(
                    "no local gguf blob found for {}:{} and cloud unavailable",
                    name, tag_final
                );
                std::process::exit(1);
            }
        }
        Commands::Simple { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            let prompt = if extra.is_empty() {
                String::new()
            } else {
                extra.join(" ")
            };
            if prompt.is_empty() {
                eprintln!("missing prompt");
                std::process::exit(2);
            }
            
            // Check for TTS/audio keywords and force ollama run if detected
            let prompt_lower = prompt.to_lowercase();
            let is_tts_or_audio = prompt_lower.contains("tts") || prompt_lower.contains("audio");
            
            if is_tts_or_audio {
                // For TTS/audio, always use ollama run regardless of local availability
                if model.contains(':') {
                    let (name, tag) = split_model_tag(&model);
                    run_ollama_simple(&name, &tag, &prompt, cli.dry_run, cli.verbose);
                } else {
                    run_ollama_simple(&model, "cloud", &prompt, cli.dry_run, cli.verbose);
                }
                return;
            }
            if model.contains(':') {
                let (name, tag) = split_model_tag(&model);
                if tag.eq_ignore_ascii_case("cloud") {
                    run_ollama_simple(&name, &tag, &prompt, cli.dry_run, cli.verbose);
                } else if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                    let bin = resolve_bin("llama-cli").unwrap_or_else(|| {
                        eprintln!("llama-cli not found on PATH");
                        std::process::exit(127)
                    });
                    let mut cmd = Command::new(bin);
                    cmd.arg("-m")
                        .arg(&blob)
                        .arg("-p")
                        .arg(&prompt)
                        .arg("-no-cnv");
                    cmd.stdout(Stdio::inherit()).stderr(Stdio::null());
                    spawn_or_print(cmd, cli.dry_run);
                } else if is_cloud_model_available(&name) {
                    run_ollama_simple(&name, "cloud", &prompt, cli.dry_run, cli.verbose);
                } else {
                    eprintln!(
                        "no local gguf blob found for {}:{} and cloud unavailable",
                        name, tag
                    );
                    std::process::exit(1);
                }
            } else {
                if let Some(p) = resolve_model_ref(&model, cli.link_dir.as_ref()) {
                    let bin = resolve_bin("llama-cli").unwrap_or_else(|| {
                        eprintln!("llama-cli not found on PATH");
                        std::process::exit(127)
                    });
                    let mut cmd = Command::new(bin);
                    cmd.arg("-m").arg(&p).arg("-p").arg(&prompt).arg("-no-cnv");
                    cmd.stdout(Stdio::inherit()).stderr(Stdio::null());
                    spawn_or_print(cmd, cli.dry_run);
                } else if is_cloud_model_available(&model) {
                    run_ollama_simple(&model, "cloud", &prompt, cli.dry_run, cli.verbose);
                } else {
                    eprintln!("model not found: {}", model);
                    std::process::exit(1);
                }
            }
        }
        Commands::List => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            println!("");
            println!("Ollama models (local):");
            println!("");
            run_ollama_list(cli.dry_run, cli.verbose);
            println!("");
            println!("llama.cpp models (.gguf):");
            println!("");
            let mut linked = 0usize;
            for p in find_llama_cache_models() {
                println!("{}", p.display());
                if symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force, cli.verbose).is_ok() {
                    linked += 1;
                }
            }
            println!("");
            for (name, tag) in enumerate_ollama_library_models() {
                if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                    let link_name = format!("{}-{}.gguf", name, tag);
                    let _ = symlink_named_into_guffy(
                        &blob,
                        &link_name,
                        cli.link_dir.as_ref(),
                        cli.force,
                        cli.verbose,
                    )
                    .map(|_| {
                        linked += 1;
                    });
                }
            }
            let link_dir = ggufy_models_dir_with(cli.link_dir.as_ref());
            println!("linked {} models into {}", linked, link_dir.display());
        }
        Commands::OllamaRun { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.ollama || cli.llamacpp {
                eprintln!("use -o for ollama mode");
                std::process::exit(2);
            }
            let (name, tag) = split_model_tag(&model);
            if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                symlink_named_into_guffy(
                    &blob,
                    &format!("{}-{}.gguf", name, tag),
                    cli.link_dir.as_ref(),
                    cli.force,
                    cli.verbose,
                )
                .ok();
                let port = port_or_default(true, cli.port);
                run_llama_server_model(&blob, Some(port), Some(&extra), cli.dry_run, cli.verbose);
            } else {
                eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                std::process::exit(1);
            }
        }
        Commands::Serve { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.llamacpp || cli.ollama {
                eprintln!("use -c for llama.cpp mode");
                std::process::exit(2);
            }
            let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                .unwrap_or_else(|| PathBuf::from(&model));
            let port = port_or_default(false, cli.port);
            run_llama_server_model(&target, Some(port), Some(&extra), cli.dry_run, cli.verbose);
        }
        Commands::Cli { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.llamacpp || cli.ollama {
                eprintln!("use -c for llama.cpp mode");
                std::process::exit(2);
            }
            let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                .unwrap_or_else(|| PathBuf::from(&model));
            run_llama_cli_model(&target, Some(&extra), cli.dry_run, cli.verbose);
        }
        Commands::Locate { pattern } => {
            let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".*").unwrap());
            for p in find_llama_cache_models()
                .into_iter()
                .chain(find_ollama_blob_gguf())
            {
                let s = p.to_string_lossy().to_string();
                if re.is_match(&s) {
                    println!("{}", s);
                }
            }
        }
        Commands::Link => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            for p in find_llama_cache_models()
                .into_iter()
                .chain(find_ollama_blob_gguf())
            {
                let _ = symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force, cli.verbose);
            }
            println!("link refresh complete");
        }
        Commands::OllamaServe { target, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.ollama || cli.llamacpp {
                eprintln!("use -o for ollama mode");
                std::process::exit(2);
            }
            let port = port_or_default(true, cli.port);
            if target.contains(':') {
                let (name, tag) = split_model_tag(&target);
                if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                    symlink_named_into_guffy(
                        &blob,
                        &format!("{}-{}.gguf", name, tag),
                        cli.link_dir.as_ref(),
                        cli.force,
                        cli.verbose,
                    )
                    .ok();
                    run_llama_server_model(
                        &blob,
                        Some(port),
                        Some(&extra),
                        cli.dry_run,
                        cli.verbose,
                    );
                } else {
                    eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                    std::process::exit(1);
                }
            } else {
                let target_path = resolve_model_ref(&target, cli.link_dir.as_ref())
                    .unwrap_or_else(|| PathBuf::from(&target));
                run_llama_server_model(
                    &target_path,
                    Some(port),
                    Some(&extra),
                    cli.dry_run,
                    cli.verbose,
                );
            }
        }
        Commands::Ollama { sub } => match sub {
            OllamaCmd::Serve { target, extra } => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let port = port_or_default(true, cli.port);
                if target.contains(':') {
                    let (name, tag) = split_model_tag(&target);
                    if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                        symlink_named_into_guffy(
                            &blob,
                            &format!("{}-{}.gguf", name, tag),
                            cli.link_dir.as_ref(),
                            cli.force,
                            cli.verbose,
                        )
                        .ok();
                        run_llama_server_model(
                            &blob,
                            Some(port),
                            Some(&extra),
                            cli.dry_run,
                            cli.verbose,
                        );
                    } else {
                        eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                        std::process::exit(1);
                    }
                } else {
                    let target_path = resolve_model_ref(&target, cli.link_dir.as_ref())
                        .unwrap_or_else(|| PathBuf::from(&target));
                    run_llama_server_model(
                        &target_path,
                        Some(port),
                        Some(&extra),
                        cli.dry_run,
                        cli.verbose,
                    );
                }
            }
            OllamaCmd::Run { model, extra } => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let port = port_or_default(true, cli.port);
                let (name, tag) = split_model_tag(&model);
                if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                    symlink_named_into_guffy(
                        &blob,
                        &format!("{}-{}.gguf", name, tag),
                        cli.link_dir.as_ref(),
                        cli.force,
                        cli.verbose,
                    )
                    .ok();
                    run_llama_server_model(
                        &blob,
                        Some(port),
                        Some(&extra),
                        cli.dry_run,
                        cli.verbose,
                    );
                } else {
                    eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                    std::process::exit(1);
                }
            }
            OllamaCmd::LinkAll => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let mut linked = 0usize;
                for (name, tag) in enumerate_ollama_library_models() {
                    if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                        let link_name = format!("{}-{}.gguf", name, tag);
                        if symlink_named_into_guffy(
                            &blob,
                            &link_name,
                            cli.link_dir.as_ref(),
                            cli.force,
                            cli.verbose,
                        )
                        .is_ok()
                        {
                            linked += 1;
                        }
                    }
                }
                println!("linked {} ollama models into ~/.guffy/models", linked);
            }
        },
        Commands::Llama { sub } => match sub {
            LlamaCmd::Hf { repo, extra } => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let port = port_or_default(false, cli.port);
                run_llama_server_hf(&repo, Some(port), Some(&extra), cli.dry_run, cli.verbose);
                if let Some(p) = find_hf_cached_gguf(&repo) {
                    symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force, cli.verbose).ok();
                }
            }
            LlamaCmd::Serve { model, extra } => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                    .unwrap_or_else(|| PathBuf::from(&model));
                let port = port_or_default(false, cli.port);
                run_llama_server_model(&target, Some(port), Some(&extra), cli.dry_run, cli.verbose);
            }
            LlamaCmd::Cli { model, extra } => {
                ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                    .unwrap_or_else(|| PathBuf::from(&model));
                run_llama_cli_model(&target, Some(&extra), cli.dry_run, cli.verbose);
            }
        },
    }
}

fn ensure_models_dir(link_override: Option<&PathBuf>) -> IoResult<()> {
    let p = ggufy_models_dir_with(link_override);
    fs::create_dir_all(&p)
}

// removed unused ggufy_models_dir to satisfy clippy dead_code

fn ggufy_models_dir_with(link_override: Option<&PathBuf>) -> PathBuf {
    if let Some(o) = link_override {
        return o.clone();
    }
    let mut p = home_dir().expect("home");
    p.push(".guffy/models");
    p
}

fn llama_cache_dir() -> PathBuf {
    if let Ok(override_dir) = std::env::var("LLAMA_CPP_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }
    #[cfg(target_os = "macos")]
    {
        let mut p = home_dir().expect("home");
        p.push("Library/Caches/llama.cpp");
        p
    }
    #[cfg(target_os = "linux")]
    {
        let mut p = home_dir().expect("home");
        p.push(".cache/llama.cpp");
        p
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
            let mut p = PathBuf::from(local_app);
            p.push("llama.cpp");
            p
        } else {
            PathBuf::from("C:/Users/Public/AppData/Local/llama.cpp")
        }
    }
}

fn ollama_blobs_dir() -> PathBuf {
    let mut p = home_dir().expect("home");
    p.push(".ollama/models/blobs");
    p
}

fn ollama_library_manifest_dir() -> PathBuf {
    let mut p = home_dir().expect("home");
    p.push(".ollama/models/manifests/registry.ollama.ai/library");
    p
}

fn symlink_into_guffy(
    src: &Path,
    link_override: Option<&PathBuf>,
    force: bool,
    verbose: bool,
) -> IoResult<()> {
    let name = src.file_name().unwrap().to_string_lossy().to_string();
    let mut dst = ggufy_models_dir_with(link_override);
    dst.push(name);
    if dst.exists() {
        if force {
            let _ = fs::remove_file(&dst);
        } else {
            return Ok(());
        }
    }
    match symlink_file(src, &dst) {
        Ok(()) => Ok(()),
        Err(e) => {
            if verbose {
                eprintln!(
                    "symlink failed: {} -> {} ({}) — falling back to copy",
                    src.display(),
                    dst.display(),
                    e
                );
            }
            fs::copy(src, &dst).map(|_| ())
        }
    }
}

fn symlink_named_into_guffy(
    src: &Path,
    link_name: &str,
    link_override: Option<&PathBuf>,
    force: bool,
    verbose: bool,
) -> IoResult<()> {
    let mut dst = ggufy_models_dir_with(link_override);
    dst.push(link_name);
    if dst.exists() {
        if force {
            let _ = fs::remove_file(&dst);
        } else {
            return Ok(());
        }
    }
    match symlink_file(src, &dst) {
        Ok(()) => Ok(()),
        Err(e) => {
            if verbose {
                eprintln!(
                    "symlink failed: {} -> {} ({}) — falling back to copy",
                    src.display(),
                    dst.display(),
                    e
                );
            }
            fs::copy(src, &dst).map(|_| ())
        }
    }
}

fn find_llama_cache_models() -> Vec<PathBuf> {
    let root = llama_cache_dir();
    let mut v = Vec::new();
    if root.exists() {
        for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if e.file_type().is_file() {
                let p = e.path().to_path_buf();
                if p.extension().map(|x| x == "gguf").unwrap_or(false) {
                    v.push(p);
                }
            }
        }
    }
    v
}

fn find_ollama_blob_gguf() -> Vec<PathBuf> {
    let root = ollama_blobs_dir();
    let mut v = Vec::new();
    if root.exists() {
        for e in WalkDir::new(root)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if e.file_type().is_file() {
                let p = e.path().to_path_buf();
                if is_gguf_file(&p).unwrap_or(false) {
                    v.push(p);
                }
            }
        }
    }
    v
}

fn is_gguf_file(p: &Path) -> IoResult<bool> {
    let mut f = fs::File::open(p)?;
    let mut head = [0u8; 4];
    let n = f.read(&mut head)?;
    Ok(n >= 4 && &head == b"GGUF")
}

fn run_llama_server_hf(
    repo: &str,
    port: Option<u16>,
    extra: Option<&[String]>,
    dry_run: bool,
    verbose: bool,
) {
    let bin = resolve_bin("llama-server").unwrap_or_else(|| {
        eprintln!("llama-server not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("-hf").arg(repo);
    let p = port.unwrap_or(12434);
    cmd.arg("--port").arg(p.to_string());
    if let Some(args) = extra {
        cmd.args(args);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if verbose {
        eprintln!("starting: llama-server -hf {}", repo);
    }
    spawn_or_print(cmd, dry_run);
}

fn run_llama_server_model(
    model_path: &Path,
    port: Option<u16>,
    extra: Option<&[String]>,
    dry_run: bool,
    verbose: bool,
) {
    let bin = resolve_bin("llama-server").unwrap_or_else(|| {
        eprintln!("llama-server not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("-m").arg(model_path);
    let p = port.unwrap_or(12434);
    cmd.arg("--port").arg(p.to_string());
    if let Some(args) = extra {
        cmd.args(args);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if verbose {
        eprintln!("starting: llama-server -m {}", model_path.display());
    }
    spawn_or_print(cmd, dry_run);
}

fn run_llama_cli_model(model_path: &Path, extra: Option<&[String]>, dry_run: bool, verbose: bool) {
    let bin = resolve_bin("llama-cli").unwrap_or_else(|| {
        eprintln!("llama-cli not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("-m").arg(model_path);
    if let Some(args) = extra {
        cmd.args(args);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if verbose {
        eprintln!("starting: llama-cli -m {}", model_path.display());
    }
    spawn_or_print(cmd, dry_run);
}

fn run_ollama_list(dry_run: bool, verbose: bool) {
    let bin = resolve_bin("ollama").unwrap_or_else(|| {
        eprintln!("ollama not found on PATH");
        std::process::exit(127)
    });
    if dry_run {
        println!("{} {}", bin.display(), "list");
        return;
    }
    let out = Command::new(bin).arg("list").output().expect("ollama list");
    if verbose {
        eprintln!("ollama list completed");
    }
    let mut buf = out.stdout;
    if !buf.ends_with(&[b'\n']) {
        buf.push(b'\n');
    }
    print!("{}", String::from_utf8_lossy(&buf));
}

fn is_cloud_model_available(model: &str) -> bool {
    let bin = match resolve_bin("ollama") {
        Some(b) => b,
        None => return false,
    };
    let mut cmd = Command::new(bin);
    cmd.arg("show").arg(format!("{}:cloud", model));
    if let Ok(status) = cmd.status() {
        return status.success();
    }
    false
}

fn normalize_model_tag(target: &str, tag: Option<&str>) -> (String, String) {
    if target.contains(':') {
        let (n, t) = split_model_tag(target);
        return (n, t);
    }
    let name = target.to_string();
    let t = tag
        .map(|s| s.to_string())
        .unwrap_or_else(|| "latest".to_string());
    (name, t)
}

fn run_ollama_run(model: &str, tag: &str, extra: Option<&[String]>, dry_run: bool, verbose: bool) {
    let bin = resolve_bin("ollama").unwrap_or_else(|| {
        eprintln!("ollama not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("run").arg(format!("{}:{}", model, tag));
    if let Some(args) = extra {
        cmd.args(args);
    }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if verbose {
        eprintln!("starting: ollama run {}:{}", model, tag);
    }
    spawn_or_print(cmd, dry_run);
}

fn run_ollama_simple(model: &str, tag: &str, prompt: &str, dry_run: bool, verbose: bool) {
    let bin = resolve_bin("ollama").unwrap_or_else(|| {
        eprintln!("ollama not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("run").arg(format!("{}:{}", model, tag)).arg(prompt);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    if verbose {
        eprintln!("starting: ollama run {}:{} \"{}\"", model, tag, prompt);
    }
    spawn_or_print(cmd, dry_run);
}

fn find_hf_cached_gguf(repo: &str) -> Option<PathBuf> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return None;
    }
    let owner = parts[0];
    let name = parts[1];
    let root = llama_cache_dir();
    if !root.exists() {
        return None;
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if e.file_type().is_file() {
            let p = e.path();
            if p.extension().map(|x| x == "gguf").unwrap_or(false) {
                let s = p.to_string_lossy();
                if s.contains(owner) && s.contains(name) {
                    candidates.push(p.to_path_buf());
                }
            }
        }
    }
    candidates.sort_by_key(|p| fs::metadata(p).map(|m| m.modified().ok()).ok().flatten());
    candidates.pop()
}

fn split_model_tag(s: &str) -> (String, String) {
    let mut parts = s.split(':');
    let name = parts.next().unwrap_or("").to_string();
    let tag = parts.next().unwrap_or("latest").to_string();
    (name, tag)
}

fn resolve_ollama_library_gguf(model: &str, tag: &str) -> Option<PathBuf> {
    let mut manifest_path = ollama_library_manifest_dir();
    manifest_path.push(model);
    manifest_path.push(tag);
    if !manifest_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&manifest_path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;
    let mut blobs: Vec<PathBuf> = Vec::new();
    collect_digests(&json, &mut blobs);
    if blobs.is_empty() {
        return None;
    }
    let mut ggufs: Vec<(u64, PathBuf)> = Vec::new();
    for b in blobs {
        if is_gguf_file(&b).unwrap_or(false) {
            if let Ok(md) = fs::metadata(&b) {
                ggufs.push((md.len(), b));
            }
        }
    }
    ggufs.sort_by_key(|(len, _)| *len);
    ggufs.pop().map(|(_, p)| p)
}

fn collect_digests(v: &Value, out: &mut Vec<PathBuf>) {
    match v {
        Value::String(s) => {
            if s.starts_with("sha256:") {
                let mut p = ollama_blobs_dir();
                let fname = s.replace(":", "-");
                p.push(fname);
                if p.exists() {
                    out.push(p);
                }
            }
        }
        Value::Array(arr) => {
            for e in arr {
                collect_digests(e, out);
            }
        }
        Value::Object(map) => {
            for (_k, val) in map.iter() {
                collect_digests(val, out);
            }
        }
        _ => {}
    }
}

fn enumerate_ollama_library_models() -> Vec<(String, String)> {
    let root = ollama_library_manifest_dir();
    let mut out = Vec::new();
    if !root.exists() {
        return out;
    }
    if let Ok(models) = fs::read_dir(&root) {
        for m in models.flatten() {
            let model_name = match m.file_name().into_string() {
                Ok(s) => s,
                Err(_) => continue,
            };
            let model_dir = m.path();
            if !model_dir.is_dir() {
                continue;
            }
            if let Ok(tags) = fs::read_dir(&model_dir) {
                for t in tags.flatten() {
                    let tag_name = match t.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    let tag_path = t.path();
                    if tag_path.is_file() {
                        out.push((model_name.clone(), tag_name));
                    }
                }
            }
        }
    }
    out
}

fn resolve_model_ref(model: &str, link_override: Option<&PathBuf>) -> Option<PathBuf> {
    let mut p = ggufy_models_dir_with(link_override);
    p.push(model);
    if p.exists() {
        return Some(p);
    }
    // allow shorthand without .gguf
    let mut p2 = ggufy_models_dir_with(link_override);
    p2.push(model);
    if p2.extension().is_none() {
        p2.set_extension("gguf");
    }
    if p2.exists() {
        return Some(p2);
    }
    None
}

fn find_mmproj_for(model_path: &Path, link_override: Option<&PathBuf>) -> Option<PathBuf> {
    // Try same directory first
    if let Some(dir) = model_path.parent() {
        if let Some(mp) = scan_for_mmproj(dir) {
            return Some(mp);
        }
    }
    // Then try link dir
    let link_dir = ggufy_models_dir_with(link_override);
    scan_for_mmproj(&link_dir)
}

fn scan_for_mmproj(dir: &Path) -> Option<PathBuf> {
    if !dir.exists() {
        return None;
    }
    for e in WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = e.path();
        if p.file_name()
            .map(|n| n.to_string_lossy().contains("mmproj"))
            .unwrap_or(false)
            && p.extension().map(|x| x == "gguf").unwrap_or(false)
        {
            return Some(p.to_path_buf());
        }
    }
    None
}

fn port_or_default(is_ollama: bool, requested: Option<u16>) -> u16 {
    if let Some(p) = requested {
        return p;
    }
    if is_ollama {
        11434
    } else {
        12434
    }
}

fn symlink_file(src: &Path, dst: &Path) -> IoResult<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(src, dst)
    }
}

fn resolve_bin(name: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(if cfg!(windows) { ';' } else { ':' }) {
            let mut candidate = PathBuf::from(p);
            candidate.push(name);
            if cfg!(windows) {
                if candidate.exists() {
                    return Some(candidate);
                }
                let mut exe = candidate.clone();
                exe.set_extension("exe");
                if exe.exists() {
                    return Some(exe);
                }
            } else if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}

#[allow(clippy::zombie_processes)]
fn spawn_or_print(mut cmd: Command, dry_run: bool) {
    if dry_run {
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        let prog = cmd.get_program().to_string_lossy().to_string();
        println!("{} {}", prog, args.join(" "));
        return;
    }
    let _ = cmd.spawn().expect("spawn");
}
