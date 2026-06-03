class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "scode-v0.3.21"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/vscode-v0.3.21/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "aa9db6a5670b92d9c99653702821cfdad349435ac589addaffd87f6cb5e5dad4"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/vscode-v0.3.21/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "ef610c8b33f3a1088e58441b8c91621dd8f90647aa88a12c1273261f6e812343"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/vscode-v0.3.21/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "e55a315cca44d18e6b955777758bff643db14d7aaf43e20deebe03096269b996"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS vscode-v0.3.21", shell_output("#{bin}/cpos help 2>&1")
  end
end
