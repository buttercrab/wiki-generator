set -ex

rustup default nightly
cargo build --release
cd target/release

bin="wiki-generator-linux.tar.gz"
tar czf ../../"$bin" wiki-generator

cd ../..

TAG=${GITHUB_REF#*/tags/}
hub release edit -m "" --attach "$bin" "$TAG"
