# Fibonacci cache

This demo calculates the nth number of fibonacci numbers and caches the result in Redis.

## How to run

```bash
cargo run --bin fibonacci_cache [n] [cache]
```

Where:

- `n` the nth number of fibonacci numbers to calculate
- `cache` wether to use the cache or not

Example:

```bash
cargo run --bin fibonacci_cache 28 true
```