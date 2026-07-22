class Pb < Formula
  desc "CLI em Rust para as APIs do PagBank"
  homepage "https://github.com/vinycalves/pagbank-cli"
  version "__VERSION__"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/vinycalves/pagbank-cli/releases/download/v__VERSION__/pb-aarch64-apple-darwin.tar.gz"
      sha256 "__SHA_AARCH64_DARWIN__"
    end
    on_intel do
      url "https://github.com/vinycalves/pagbank-cli/releases/download/v__VERSION__/pb-x86_64-apple-darwin.tar.gz"
      sha256 "__SHA_X86_64_DARWIN__"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/vinycalves/pagbank-cli/releases/download/v__VERSION__/pb-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "__SHA_AARCH64_LINUX__"
    end
    on_intel do
      url "https://github.com/vinycalves/pagbank-cli/releases/download/v__VERSION__/pb-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "__SHA_X86_64_LINUX__"
    end
  end

  def install
    bin.install "pb"
  end

  test do
    assert_match "pb #{version}", shell_output("#{bin}/pb --version")
  end
end
