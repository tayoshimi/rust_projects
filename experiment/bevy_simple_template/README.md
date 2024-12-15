## usage

### 事前準備

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

### build

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_app \
  --out-dir wasm/js \
  --target web target/wasm32-unknown-unknown/release/bevy_app.wasm
```

### execute

The first command will build the example for the wasm target, creating a binary. Then,
[wasm-bindgen-cli](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) is used to create
javascript bindings to this wasm file in the output file `wasm/target/wasm_example.js`, which can be loaded using this
[example HTML file](./wasm/index.html).

Then serve `wasm` directory to browser. i.e.

```sh
# cargo install basic-http-server
basic-http-server wasm

# with python
python3 -m http.server --directory wasm

# with ruby
ruby -run -ehttpd wasm
```


## ビルドできない時情報

[Bevy-reflect: cannot call non-const fn in constant functions #9374](https://github.com/bevyengine/bevy/issues/9374)

```sh
cargo update + cargo clean
```
