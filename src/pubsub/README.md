# Pub/Sub

This is a simple chat client to demonstrate the use of [redis pub/sub](https://redis.io/docs/interact/pubsub/).
It spawns two tokio tasks which can be thought of like threads, one for publishing and one for subscribing to a channel.

## How to run

```bash
cargo run --bin pubsub
```

Upon running the command, you will be prompted to enter a channel to which you want to subscribe and send messages to.
After that, you can enter messages to send to the channel, and you will see the messages you sent and received.

## Practical use case

https://github.com/MCloudTT/mcloudtt