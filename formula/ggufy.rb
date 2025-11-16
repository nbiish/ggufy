class Ggufy < Formula
  desc "Unified GGUF wrapper for llama.cpp and Ollama"
  homepage "https://github.com/nbiish/ggufy"
  url "https://github.com/nbiish/ggufy/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "b33fb644c7a1c6aa8a7be1a8b836d5a7c29f8b67ae1464e9e550405f05c7e236"
  version "0.1.1"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/ggufy"
    if File.exist?("target/release/ggufy-simple")
      bin.install "target/release/ggufy-simple"
    end
  end

  test do
    system "#{bin}/ggufy", "list"
    system "#{bin}/ggufy-simple", "--help"
  end
end