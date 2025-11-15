class Ggufy < Formula
  desc "Unified GGUF wrapper for llama.cpp and Ollama"
  homepage "https://github.com/nbiish/ggufy"
  url "https://github.com/nbiish/ggufy.git", :using => :git, :tag => "v0.1.0"
  version "0.1.0"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/ggufy"
  end

  test do
    system "#{bin}/ggufy", "list"
  end
end