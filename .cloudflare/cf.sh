curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
~/.cargo/bin/cargo doc
cp ./cloudflare/index.html ./target/doc/
