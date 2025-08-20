class Rtbt < Formula
  desc "Blazing-fast, memory-efficient CLI tool for converting images to themed color palettes"
  homepage "https://github.com/emgeedata/rustbucket"
  url "https://github.com/emgeedata/rustbucket/archive/v1.0.1.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    # Build the main binary
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    
    # Generate shell completions
    system "cargo", "run", "--bin", "generate-completions", "--release"
    
    # Install shell completions
    bash_completion.install "completions/bash/rtbt.bash" => "rtbt"
    zsh_completion.install "completions/zsh/_rtbt"
    fish_completion.install "completions/fish/rtbt.fish"
    
    # Install man page
    man1.install "docs/man/rtbt.1"
  end

  test do
    # Test basic functionality
    assert_match "rtbt", shell_output("#{bin}/rtbt --help")
    assert_match "1.0.0", shell_output("#{bin}/rtbt --version")
    
    # Test palette listing
    assert_match "nord", shell_output("#{bin}/rtbt --list-palettes")
    assert_match "dracula", shell_output("#{bin}/rtbt --list-palettes")
  end
end
