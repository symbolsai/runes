This NodeJS package is a small collection of helpers written on top of the [ordinals Rust crate](https://docs.rs/ordinals/latest/ordinals/index.html) that allows you to do basic parsing and validating of raw transactions.

### Building locally

If you want to build this project locally and generate the JS/WASM yourself you'll need to make sure a few things are setup:

- [Rust](https://www.rust-lang.org/tools/install)
- clang:
  - Linux: `apt install clang`
  - Mac (DOT NOT use the default version xcode installs): `brew install llvm`
- wasm-pack: `cargo install wasm-pack`
- MacOS only:
  Put the following in your `.zshrc`/`.bashrc` and source it:

  ```bash
  AR=/opt/homebrew/opt/llvm/bin/llvm-ar
  CC=/opt/homebrew/opt/llvm/bin/clang
  ```

### Build

```bash
./build.sh
```
