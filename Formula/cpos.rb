class Cpos < Formula
  desc "Competitive Programming Operating System terminal app"
  homepage "https://github.com/Soham109/cpos"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.0/cpos-aarch64-apple-darwin.tar.gz"
      sha256 "1f7b2963177b96130a7c1c454bd6cb823d85d02277da4010cd603338446807df"
    end

    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.0/cpos-x86_64-apple-darwin.tar.gz"
      sha256 "d275590cf03bdbf32ce4e11e93eee579c407d55dbdeb4ee67c8c8321a18b9348"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/Soham109/cpos/releases/download/v0.1.0/cpos-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "cb85a215971ad3bddc6db1d0dcc9f9d01d9ec1cdd9a8a2dcf8d64189fab01e37"
    end
  end

  def install
    bin.install "cpos"
  end

  test do
    assert_match "CPOS v0.1.0", shell_output("#{bin}/cpos help 2>&1")
  end
end
