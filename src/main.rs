use clap::{ArgAction, Parser, Subcommand};
use dirs::home_dir;
use serde_json::Value;
use std::fs;
use std::io::{Read, Result as IoResult};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use regex::Regex;

#[derive(Parser)]
#[command(name = "ggufy", version, about = "Wraps llama.cpp and Ollama for GGUF management")]
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
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hf { repo: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    List,
    OllamaRun { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Serve { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Cli { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Locate { pattern: String },
    Link,
    OllamaServe { target: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Ollama { #[command(subcommand)] sub: OllamaCmd },
    Llama { #[command(subcommand)] sub: LlamaCmd },
}

#[derive(Subcommand)]
enum OllamaCmd {
    Serve { target: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Run { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
}

#[derive(Subcommand)]
enum LlamaCmd {
    Hf { repo: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Serve { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
    Cli { model: String, #[arg(trailing_var_arg = true)] extra: Vec<String> },
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
            run_llama_server_hf(&repo, Some(port), Some(&extra));
            if let Some(p) = find_hf_cached_gguf(&repo) {
                symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force).ok();
            }
        }
        Commands::List => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            let mut linked = 0usize;
            for p in find_llama_cache_models() {
                if symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force).is_ok() {
                    linked += 1;
                }
            }
            for p in find_ollama_blob_gguf() {
                if symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force).is_ok() {
                    linked += 1;
                }
            }
            println!("linked {} models into ~/.guffy/models", linked);
        }
        Commands::OllamaRun { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.ollama || cli.llamacpp {
                eprintln!("use -o for ollama mode");
                std::process::exit(2);
            }
            let (name, tag) = split_model_tag(&model);
            if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                symlink_named_into_guffy(&blob, &format!("{}-{}.gguf", name, tag), cli.link_dir.as_ref(), cli.force).ok();
                let port = port_or_default(true, cli.port);
                run_llama_server_model(&blob, find_mmproj_for(&blob, cli.link_dir.as_ref()), Some(port), Some(&extra));
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
            run_llama_server_model(&target, find_mmproj_for(&target, cli.link_dir.as_ref()), Some(port), Some(&extra));
        }
        Commands::Cli { model, extra } => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            if !cli.llamacpp || cli.ollama {
                eprintln!("use -c for llama.cpp mode");
                std::process::exit(2);
            }
            let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                .unwrap_or_else(|| PathBuf::from(&model));
            run_llama_cli_model(&target, Some(&extra));
        }
        Commands::Locate { pattern } => {
            let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".*").unwrap());
            for p in find_llama_cache_models().into_iter().chain(find_ollama_blob_gguf()) {
                let s = p.to_string_lossy().to_string();
                if re.is_match(&s) {
                    println!("{}", s);
                }
            }
        }
        Commands::Link => {
            ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
            for p in find_llama_cache_models().into_iter().chain(find_ollama_blob_gguf()) {
                let _ = symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force);
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
                    symlink_named_into_guffy(&blob, &format!("{}-{}.gguf", name, tag), cli.link_dir.as_ref(), cli.force).ok();
                    run_llama_server_model(&blob, find_mmproj_for(&blob, cli.link_dir.as_ref()), Some(port), Some(&extra));
                } else {
                    eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                    std::process::exit(1);
                }
            } else {
                let target_path = resolve_model_ref(&target, cli.link_dir.as_ref())
                    .unwrap_or_else(|| PathBuf::from(&target));
                run_llama_server_model(&target_path, find_mmproj_for(&target_path, cli.link_dir.as_ref()), Some(port), Some(&extra));
            }
        }
        Commands::Ollama { sub } => {
            match sub {
                OllamaCmd::Serve { target, extra } => {
                    ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                    let port = port_or_default(true, cli.port);
                    if target.contains(':') {
                        let (name, tag) = split_model_tag(&target);
                        if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                            symlink_named_into_guffy(&blob, &format!("{}-{}.gguf", name, tag), cli.link_dir.as_ref(), cli.force).ok();
                            run_llama_server_model(&blob, find_mmproj_for(&blob, cli.link_dir.as_ref()), Some(port), Some(&extra));
                        } else {
                            eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                            std::process::exit(1);
                        }
                    } else {
                        let target_path = resolve_model_ref(&target, cli.link_dir.as_ref())
                            .unwrap_or_else(|| PathBuf::from(&target));
                        run_llama_server_model(&target_path, find_mmproj_for(&target_path, cli.link_dir.as_ref()), Some(port), Some(&extra));
                    }
                }
                OllamaCmd::Run { model, extra } => {
                    ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                    let port = port_or_default(true, cli.port);
                    let (name, tag) = split_model_tag(&model);
                    if let Some(blob) = resolve_ollama_library_gguf(&name, &tag) {
                        symlink_named_into_guffy(&blob, &format!("{}-{}.gguf", name, tag), cli.link_dir.as_ref(), cli.force).ok();
                        run_llama_server_model(&blob, find_mmproj_for(&blob, cli.link_dir.as_ref()), Some(port), Some(&extra));
                    } else {
                        eprintln!("no gguf blob found for {}:{} in ollama library", name, tag);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Llama { sub } => {
            match sub {
                LlamaCmd::Hf { repo, extra } => {
                    ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                    let port = port_or_default(false, cli.port);
                    run_llama_server_hf(&repo, Some(port), Some(&extra));
                    if let Some(p) = find_hf_cached_gguf(&repo) {
                        symlink_into_guffy(&p, cli.link_dir.as_ref(), cli.force).ok();
                    }
                }
                LlamaCmd::Serve { model, extra } => {
                    ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                    let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                        .unwrap_or_else(|| PathBuf::from(&model));
                    let port = port_or_default(false, cli.port);
                    run_llama_server_model(&target, find_mmproj_for(&target, cli.link_dir.as_ref()), Some(port), Some(&extra));
                }
                LlamaCmd::Cli { model, extra } => {
                    ensure_models_dir(cli.link_dir.as_ref()).expect("models dir");
                    let target = resolve_model_ref(&model, cli.link_dir.as_ref())
                        .unwrap_or_else(|| PathBuf::from(&model));
                    run_llama_cli_model(&target, Some(&extra));
                }
            }
        }
    }
}

fn ensure_models_dir(link_override: Option<&PathBuf>) -> IoResult<()> {
    let p = ggufy_models_dir_with(link_override);
    fs::create_dir_all(&p)
}

fn ggufy_models_dir() -> PathBuf {
    ggufy_models_dir_with(None)
}

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
        return p;
    }
    #[cfg(target_os = "linux")]
    {
        let mut p = home_dir().expect("home");
        p.push(".cache/llama.cpp");
        return p;
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
            let mut p = PathBuf::from(local_app);
            p.push("llama.cpp");
            return p;
        }
        return PathBuf::from("C:/Users/Public/AppData/Local/llama.cpp");
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

fn symlink_into_guffy(src: &Path, link_override: Option<&PathBuf>, force: bool) -> IoResult<()> {
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
    std::os::unix::fs::symlink(src, dst)
}

fn symlink_named_into_guffy(src: &Path, link_name: &str, link_override: Option<&PathBuf>, force: bool) -> IoResult<()> {
    let mut dst = ggufy_models_dir_with(link_override);
    dst.push(link_name);
    if dst.exists() {
        if force {
            let _ = fs::remove_file(&dst);
        } else {
            return Ok(());
        }
    }
    std::os::unix::fs::symlink(src, dst)
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
        for e in WalkDir::new(root).max_depth(1).into_iter().filter_map(|e| e.ok()) {
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

fn run_llama_server_hf(repo: &str, port: Option<u16>, extra: Option<&[String]>) {
    let mut cmd = Command::new("llama-server");
    cmd.arg("-hf").arg(repo);
    let p = port.unwrap_or(12434);
    cmd.arg("--port").arg(p.to_string());
    if let Some(args) = extra { cmd.args(args); }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let _ = cmd.spawn().expect("llama-server");
}

fn run_llama_server_model(model_path: &Path, mmproj: Option<PathBuf>, port: Option<u16>, extra: Option<&[String]>) {
    let mut cmd = Command::new("llama-server");
    cmd.arg("-m").arg(model_path);
    if let Some(mp) = mmproj {
        cmd.arg("--mmproj").arg(mp);
    }
    let p = port.unwrap_or(12434);
    cmd.arg("--port").arg(p.to_string());
    if let Some(args) = extra { cmd.args(args); }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let _ = cmd.spawn().expect("llama-server");
}

fn run_llama_cli_model(model_path: &Path, extra: Option<&[String]>) {
    let mut cmd = Command::new("llama-cli");
    cmd.arg("-m").arg(model_path);
    if let Some(args) = extra { cmd.args(args); }
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let _ = cmd.spawn().expect("llama-cli");
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

fn resolve_model_ref(model: &str, link_override: Option<&PathBuf>) -> Option<PathBuf> {
    let mut p = ggufy_models_dir_with(link_override);
    p.push(model);
    if p.exists() {
        return Some(p);
    }
    // allow shorthand without .gguf
    let mut p2 = ggufy_models_dir_with(link_override);
    p2.push(format!("{}", model));
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
    if !dir.exists() { return None; }
    for e in WalkDir::new(dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let p = e.path();
        if p.file_name().map(|n| n.to_string_lossy().contains("mmproj")).unwrap_or(false) {
            if p.extension().map(|x| x == "gguf").unwrap_or(false) {
                return Some(p.to_path_buf());
            }
        }
    }
    None
}

fn port_or_default(is_ollama: bool, requested: Option<u16>) -> u16 {
    if let Some(p) = requested { return p; }
    if is_ollama { 11434 } else { 12434 }
}