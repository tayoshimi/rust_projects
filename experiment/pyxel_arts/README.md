## ビルド方法

'''
cargo run --example <example_name>
'''

<example_name>は、Cargo.tomlに記述されている -> [[example]] name = "bit_composition_01"



## linux用ビルド

いくつか手動で編集したいので、pyxelを全部落としてくる

```sh
git clone https://github.com/kitao/pyxel.git
```

### Cargo.toml

```toml
[dependencies]
pyxel-engine = { path = "pyxel/rust/pyxel-engine", version = "2.2.8" }
```

### build時Errorの対処

#### clang関連Lib

```sh
--- stdout
  cargo:rustc-flags=-l SDL2

  --- stderr
  thread 'main' panicked at /home/tayoshimi/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bindgen-0.71.1/lib.rs:604:27:
  Unable to find libclang: "couldn't find any valid shared libraries matching: ['libclang.so', 'libclang-*.so', 'libclang.so.*', 'libclang-*.so.*'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
```

```sh
sudo pacman -S clang
```

Ubuntu

```sh
 sudo apt install libclang-dev
```

#### SDL2関連

```sh
  --- stdout
  cargo:rustc-flags=-l SDL2

  --- stderr
  wrapper.h:1:10: fatal error: 'SDL.h' file not found
  thread 'main' panicked at pyxel/rust/pyxel-platform/build.rs:220:14:
  Failed to generate bindings: ClangDiagnostic("wrapper.h:1:10: fatal error: 'SDL.h' file not found\n")
```

SDL2.h(及び関連ヘッダー)が足りていない

ARCH　Linuxの場合

SDL2の追加と編集

```sh
sudo pacman -S sdl2
```

Ubuntu

```sh
sudo apt install libsdl2-dev
```

```
./pyxel/rust/pyxel-platform/wrapper.h
#include <SDL.h> -> #include <SDL2/SDL.h>
```