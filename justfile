build:
  cargo build --target=wasm32-wasi --release

copy:
  @echo 'Copying built file...'
  cp ./target/wasm32-wasi/release/xmas_tree.wasm .


run:
  wasmer run --dir=. --enable-all ./xmas_tree.wasm
