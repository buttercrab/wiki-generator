set -ex

rustup default nightly
cargo build --release
cd target/release

bin="wiki-generator-$1.tar.gz"
tar czf ../../"$bin" mdbook

cd ../..

TAG=${GITHUB_REF#*/tags/}
hub release edit -m "" --attach "$bin" "$TAG"
