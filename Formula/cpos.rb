class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "0.1.2"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.2/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "68fd98c2994c43036b60209eef81ecdfd945a734709b8e4b9597ece35e9e48df"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.2/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "9cc6d8ff162522aa6f7e63b9b4111c55dc1a049e2460557fcf6bb02c99d2cd71"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.2/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "ea94f47598d81e97777f310f30aed263a01ee398bce7227bc8823db4337f1f06"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v0.1.2", shell_output("#{bin}/cpos help 2>&1")
  end
end
