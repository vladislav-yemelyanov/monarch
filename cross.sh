# # Linux 64-bit
cross build --release --target x86_64-unknown-linux-gnu

# # Windows 64-bit
# cross build --release --target x86_64-pc-windows-gnu

# Mac
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
