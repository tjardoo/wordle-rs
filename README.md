# Wordle Solver in Rust

YouTube video: [Implementing and Optimizing a Wordle Solver in Rust](https://www.youtube.com/watch?v=doFowk4xj7Q)

## Usage

The example below uses the `cutoff` algorithm and will play `5` games consecutively.

```sh
cargo run -- --implementation cutoff --max 5
```

## Available Algorithms

List is ordered on performance (fastest -> slowest):

- Cutoff
- Prune
- Weight
- Precalc
- Once
- Vecrem
- Allocs

## Testing

```sh
cargo test
```
