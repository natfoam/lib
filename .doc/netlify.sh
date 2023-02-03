rustup default stable
cargo doc --no-deps
cp ./.doc/index.html ./target/doc/
