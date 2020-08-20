set -ex

rustup default nightly
cargo build --release
cd target/release

case $1 in
ubuntu* | macos*)
  bin="wiki-generator-$1.tar.gz"
  tar czf ../../"$bin" mdbook
  ;;
windows*)
  bin="wiki-generator-$1.zip"
  7z a ../../"$bin" mdbook.exe
  ;;
*)
  echo "not supported os"
  ;;
esac

cd ../..

TAG=${GITHUB_REF#*/tags/}
hub release edit -m "" --attach "$bin" "$TAG"
