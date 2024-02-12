# Redis learning

## Usefull links

- [Redis commands](https://redis.io/commands)
- [Redis cli cheatsheet](https://redis.io/topics/rediscli)
- [Redis university](https://university.redis.com/)
- [RedisInsight - GUI](https://github.com/RedisInsight/RedisInsight)

## How to run

- Run docker compose

```bash
docker-compose up -d --build --remove-orphanse
```

- Connect to container

```bash
docker exec -it redis-demo fish
```

## How to use

- Use redis-cli

```bash
redis-cli -h redis
```

---

- Use demo application inside container (possible demo names: `cache`, `fibonacci_cache`, `pubsub`, `geo`)

```bash
cargo run --bin [DEMO_NAME]
```

---

The demo `fibonacci_cache` requires two arguments:
- `n` the nth number of fibonacci numbers to calculate
- `cache` wether to use the cache or not

Example:

```bash
cargo run --bin fibonacci_cache 20 true
```

---

The demo `geo` requires you to run the following to import the sample data provided by OpenStreetMap:

```bash
python3 /redis_demo/sample_data/etl.py
```

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._