# Photo filter in Rust/Wasm

## Step 1 - Dependencies

- Rust must be installed
- Set wasm target `rustup target add wasm32v1-none`
- Wasm optimizer `cargo install wasm-opt`

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install target `wasm32v1-none`, for rust
# to be capable of compiling to wasm32
rustup target add wasm32v1-none

cargo install wasm-opt --version 0.116.1 --locked --force

cargo install wasm-tools --version 1.240.0 --locked --force
```



## Step 2 - Compile WASM

#### Option 1 - Run build.sh 

Use the script `./build.sh`

```sh
./build.sh
```

#### Option 2 - Manually

Obs: make sure you are in the root of the project

```sh
# Compiles project `wasm-runtime` to WASM
set CARGO_ENCODED_RUSTFLAGS=$'-Clink-arg=-zstack-size=65536\037-Clink-arg=--import-memory\037-Ctarget-feature=+mutable-globals,-atomics,-bulk-memory,-crt-static,-exception-handling,-extended-const,-multivalue,-nontrapping-fptoint,-reference-types,-relaxed-simd,-sign-ext,-simd128,-tail-call,-wide-arithmetic';\
set SOURCE_DATE_EPOCH='1600000000';\
set TZ='UTC';\
set LC_ALL='C';\
cargo build --package=wasm-runtime --profile=release --target=wasm32v1-none --no-default-features

# Copy runtime to the root of the project
cp ./target/wasm32v1-none/release/wasm_runtime.wasm ./
```

## Step 3 - Execute WASM

Copy the file wasm_runtime to the folder "frontend"
Open the image_processor.html file
