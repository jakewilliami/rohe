cargo build
strip ./target/debug/$(basename "$PWD")
cp -f ./target/debug/$(basename "$PWD") ./
