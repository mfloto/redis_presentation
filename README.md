# Redis learning

## Usefull links

- [Redis commands](https://redis.io/commands)
- [Redis commands cheatsheet](https://redis.io/topics/rediscli)
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

- Use demo application inside container (possible demo names: `cache`, `pubsub`, `geo`)

```bash
cargo run --bin [DEMO_NAME]
```

_Hint: Before running the `geo` demo, you need to run the following to import the sample data provided by OpenStreetMap:_

```bash
python3 /redis_demo/sample_data/etl.py
```

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._