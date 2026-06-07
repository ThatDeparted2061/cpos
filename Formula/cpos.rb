class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "0.1.5"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.5/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "2d11873590bd4cb362fef857ada695be1713078d9242974c738881f5c3c35992"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.5/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "710d535d98a9030f3d9c3b712078d9c0ba09a6e12e748113f50a84ebea96c33a"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.5/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "8c466a91901b16217f40e04f0190a149f794087a2d93133e6634d9f2c2c9dfaa"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v0.1.5", shell_output("#{bin}/cpos help 2>&1")
  end
end
