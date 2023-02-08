# Wordle Solver in Rust

YouTube video: [Implementing and Optimizing a Wordle Solver in Rust](https://www.youtube.com/watch?v=doFowk4xj7Q)

## Usage

The example below uses the `cutoff` algorithm. See the directory `/src/algorithms` for all available implementations.

```rust
cargo run -- --implementation cutoff --max 5
```

## Testing

```rust
cargo test
```
