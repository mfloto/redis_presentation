# Sample data: Lidl locations in germany

## Retrieve data

Visit [https://overpass-turbo.eu/](https://overpass-turbo.eu/) to gather the locations from OpenStreetMap.

Run the following query:

```overpass
[out:json];
area["ISO3166-1"="DE"]->.searchArea;
(
  node["shop"="supermarket"]["name"="Lidl"](area.searchArea);
  way["shop"="supermarket"]["name"="Lidl"](area.searchArea);
);
out center;
```

![Locations of Lidl supermarkets in Germany](../assets/lidl_locations.png)

## Data in redis

The data of each store is stored in a hash with the key `lidl_location:<id>`, while simultaneously the location is
stored in a sorted set named `lidl_geo`. For a later demo, an index on the city is created in an unsorted set
named `lidl_index:city:<city_name>`.

---

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open
Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._