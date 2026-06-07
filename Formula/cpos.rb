class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "0.1.4"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.4/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "683faa98f0195c7b7365675828f07e513a6467185977080517b885fcb8f0fc4c"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.4/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "43a165ea8fd51937e3cd88bfa5952d9c3567ac996388c898b9611a6e36c72bb6"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.4/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "372f9ce77b1febd74ba8e774a97bbc0e6194f2f65d6137762c11255f8e7b192c"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v0.1.4", shell_output("#{bin}/cpos help 2>&1")
  end
end
