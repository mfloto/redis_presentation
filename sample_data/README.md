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

_OpenStreetMap® is open data, [licensed](https://www.openstreetmap.org/copyright) under the Open Data Commons Open Database License (ODbL) by the OpenStreetMap Foundation (OSMF)._