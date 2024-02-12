# GEORADIUS

## How to use

First, run the following to import the sample data provided by OpenStreetMap:

```bash
python3 /redis_demo/sample_data/etl.py
```

This demo requires three arguments:

- `latitude` the latitude of the center of the radius
- `longitude` the longitude of the center of the radius
- `radius` the radius in kilometers

```bash
cargo run --bin geo_radius [latitude] [longitude] [radius]
```

Example:

```bash
cargo run --bin geo_radius 49.47474553497348 8.534231778349126 3
```

---

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open
Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._