# Cache

This demonstrates the ability to cache data in Redis for a set amount of time. Using redisinsight, you can see the time
of the key `answer` expire. After the key expires, the value is recalculated and stored again.

## How to run

```bash
cargo run --bin cache
```