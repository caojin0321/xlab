ci:
    just fmt
    just clippy

clippy:
    cargo clippy --all-features --all-targets

fmt:
    cargo fmt -- --check

# requires wabt, binaryen and wasm-bindgen-cli to be installed
# all of which can be installed as linux packages
build-wasm:
    cargo build -p yoob-wasm --release --target wasm32-unknown-unknown
    wasm-bindgen \
        --target experimental-nodejs-module \
        --out-dir yoob-wasm/pkg/src/generated \
        target/wasm32-unknown-unknown/release/yoob_wasm.wasm

# build-wasm-full
bwf:
    just build-wasm && npm run build --prefix ./yoob-wasm/pkg

# Runs a command that compiles the docs then opens it as if it were the official docs on Docs.rs
# Requires nightly toolchain
doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --package yoob-core --all-features --open
