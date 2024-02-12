# Geospatial data in Redis

## How to run

First, run the following to import the sample data provided by OpenStreetMap:

```bash
python3 /redis_demo/sample_data/etl.py
```

This demo requires one argument:

- `city` the name of the city to search for. This name needs to be included in the index created by the script mentioned
  above.

```bash
cargo run --bin geo [city]
```

Example:

```bash
cargo run --bin geo mannheim
```

---

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open
Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._