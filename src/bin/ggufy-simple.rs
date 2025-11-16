use clap::Parser;
use dirs::home_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser)]
struct SimpleCli {
    #[arg(short = 'm', long = "model")]
    model_flag: Option<String>,
    #[arg(index = 1)]
    model_pos: Option<String>,
    #[arg(long = "link-dir", env = "GGUFY_MODELS_DIR")]
    link_dir: Option<String>,
    #[arg(index = 2, num_args = 1..)]
    prompt: Vec<String>,
}

fn main() {
    let cli = SimpleCli::parse();
    let prompt = if cli.prompt.is_empty() { String::new() } else { cli.prompt.join(" ") };
    if prompt.is_empty() {
        eprintln!("missing prompt");
        std::process::exit(2);
    }
    let model = cli.model_flag.clone().or(cli.model_pos.clone()).unwrap_or_else(|| {
        eprintln!("missing model");
        std::process::exit(2)
    });
    if model.contains(':') {
        let (name, tag) = split_model_tag(&model);
        if tag.eq_ignore_ascii_case("cloud") {
            run_ollama_simple(&name, &tag, &prompt);
            return;
        }
        let p = link_path_for(&format!("{}-{}.gguf", name, tag), cli.link_dir.as_deref());
        run_llama_cli_model(&p, &prompt);
    } else {
        let p = link_path_for(&model, cli.link_dir.as_deref());
        if p.exists() {
            run_llama_cli_model(&p, &prompt);
        } else if is_cloud_model_available(&model) {
            run_ollama_simple(&model, "cloud", &prompt);
        } else {
            eprintln!("model not found: {}", model);
            std::process::exit(1);
        }
    }
}

fn resolve_bin(name: &str) -> Option<String> {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(if cfg!(windows) { ';' } else { ':' }) {
            let cand = format!("{}/{}", p, name);
            if std::path::Path::new(&cand).exists() {
                return Some(cand);
            }
        }
    }
    None
}

fn default_link_dir() -> PathBuf {
    if let Ok(env) = std::env::var("GGUFY_MODELS_DIR") {
        return PathBuf::from(env);
    }
    let mut p = home_dir().expect("home");
    p.push(".guffy/models");
    p
}

fn link_path_for(model: &str, link_override: Option<&str>) -> PathBuf {
    let base = link_override.map(PathBuf::from).unwrap_or_else(default_link_dir);
    let mut p = base.join(model);
    if p.extension().is_none() {
        p.set_extension("gguf");
    }
    p
}

fn split_model_tag(s: &str) -> (String, String) {
    let mut parts = s.split(':');
    let name = parts.next().unwrap_or("").to_string();
    let tag = parts.next().unwrap_or("latest").to_string();
    (name, tag)
}

fn run_llama_cli_model(model_path: &PathBuf, prompt: &str) {
    let bin = resolve_bin("llama-cli").unwrap_or_else(|| {
        eprintln!("llama-cli not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("-m").arg(model_path).arg("-p").arg(prompt).arg("-no-cnv");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::null());
    let _ = cmd.status().expect("llama-cli");
}

fn run_ollama_simple(model: &str, tag: &str, prompt: &str) {
    let bin = resolve_bin("ollama").unwrap_or_else(|| {
        eprintln!("ollama not found on PATH");
        std::process::exit(127)
    });
    let mut cmd = Command::new(bin);
    cmd.arg("run").arg(format!("{}:{}", model, tag)).arg(prompt);
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let _ = cmd.status().expect("ollama run");
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