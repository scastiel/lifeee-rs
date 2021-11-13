# [Lifeee](https://lifeee.netlify.app) – An implementation of the Game of Life

I realized this application to keep learning [Rust](https://www.rust-lang.org/), discover the front-end library [Yew](https://yew.rs/), and because I’m a big fan of [John Conway’s Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). Please consider it always a work-in-progress.

## Features

- Draggable & zoomable **infinite grid**
- Adjustable **speed** of simulation
- Library of **patterns** extracted from the official [Lexicon](https://playgameoflife.com/lexicon)

## Work-in-progress features

- Better support for mobile (pinch-and-zoom)
- Sexier view of the pattern library (descriptions, search, etc.)
- Make the view _follow_ the displayed pattern
- Draw your own pattern on the grid
- Compose several patterns in a simulation
- Import & export RLE files

## Run locally

To start the application locally, run the following commands:

Use `nightly`:

```
rustup default nightly && rustup update
```

Install `trunk` and `wasm-bingen-cli`:

```
cargo install trunk wasm-bindgen-cli
```

Add `wasm32-unknown-unknown` target:

```
rustup target add wasm32-unknown-unknown
```

Run the application using `trunk`:

```
trunk serve
```

## Want to contribute?

Please do 😉

## License

See [LICENSE](https://github.com/scastiel/lifeee-rs/blob/main/LICENSE).
