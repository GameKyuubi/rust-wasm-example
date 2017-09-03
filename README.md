# rust-wasm-example

This project uses SDL2 and WASM.

Make sure you have SDL2 [installed](https://wiki.libsdl.org/Installation).
On Ubuntu, `sudo apt-get install libsdl2-dev` worked for me.
OSX should use `brew install sdl2` and then run `export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"`

Make sure Rust is at the latest version.
```
rustup update
rustup install nightly
```

### Building Natively
After Rust is installed, `cargo run` should compile and run the program.

### Building for Web
Make sure you have properly [installed Emscripten](http://kripken.github.io/emscripten-site/docs/getting_started/downloads.html#platform-notes-installation-instructions-sdk)

`rustup target add asmjs-unknown-emscripten`

Make sure your esmdk_env.sh is sourced!

Set these flags: `export "EMMAKEN_CFLAGS=-s USE_SDL=2"`

`cargo build --target asmjs-unknown-emscripten`
