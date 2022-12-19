# Rust Bevy Wasm Snake Game

A little project for learning Rust, to try out Bevy and compling
to a Webassembly binary.

Bevy version 0.9 compatible.

The game itself is just a basic snake game.

## Building

A simple `cargo build` is enough for the debug build.

## Webassembly build

Run `cargo build --target wasm32-unknown-unknown` to compile
the WebAssembly version of the game.

To compile the release version, run
`cargo build --release --target wasm32-unknown-unknown` and
`wasm-bindgen --out-dir ./out/ --target web ./target/`

To test it locally, `cargo run --target wasm32-unknown-unknown`
is enough start the web server that serves it.

## Playing the Game

Use arrow keys to control the snake.
At any point press `Escape` to exit the game.

On the wasm version you need to give it focus first (by clicking
it with the mouse).

## Issues

Dit issue tracker is used to track what needs to be done.
Dit issues are in a separate git repository.

## Contributors

  - me, all the code, wall sprite
  - Living tissue background: https://opengameart.org/content/living-tissue-background
  - snake logo: https://pixabay.com/users/openclipart-vectors-30363/
  - apple?
