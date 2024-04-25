### Requirements for building

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
