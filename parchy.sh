cargo build --release
upx --best --lzma target/release/parch
mv target/release/parch .
chmod +x parch
