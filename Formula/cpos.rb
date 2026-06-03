class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.1/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "64093c45445189f8819859340d89a41b2352f80e8b2c9376fec5292bcfb59192"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.1/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "2563ccebc7cb53c3055d2b465c06f310937ed1632df69be743189fee83554d65"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.1/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "00411d5c8b1bffee4adff6e0bdfbd3f1e3d1e86d35d6bfd3ce9de363c474530b"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v0.1.1", shell_output("#{bin}/cpos help 2>&1")
  end
end
