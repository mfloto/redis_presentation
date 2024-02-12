import redis
import json

# Connect to Redis
try:
    r = redis.Redis(host='redis', port=6379, db=0)
    print("Connected to Redis")
except redis.ConnectionError as e:
    print(f"Error: Could not connect to Redis: {e}")
    exit(1)

# Read lidl locations from file
included_stores = 0

try:
    with open('/redis_demo/sample_data/lidl_locations.json') as json_file:
        data = json.load(json_file)
        # Separate nodes and ways
        store_nodes = [element for element in data['elements'] if element['type'] == 'node']
        store_ways = [element for element in data['elements'] if element['type'] == 'way']

        # Add stores from nodes to Redis
        for store in store_nodes:
            # Add store to hash with store id as key and store and address as value if the store has the required tags
            if 'name' in store['tags'] and 'addr:street' in store['tags'] and 'addr:housenumber' in store[
                'tags'] and 'addr:postcode' in store['tags'] and 'addr:city' in store['tags']:
                # Add store as hash
                r.geoadd('lidl_geo', (store['lon'], store['lat'], store['id']), nx=True)
                r.hset('lidl_location:' + str(store['id']),
                       mapping={'name': store['tags']['name'], 'street': store['tags']['addr:street'],
                                'housenumber': store['tags']['addr:housenumber'],
                                'postcode': store['tags']['addr:postcode'], 'city': store['tags']['addr:city']})
                # Add store to city index
                r.sadd('lidl_index:city:' + store['tags']['addr:city'].lower(), store['id'])
                included_stores += 1

        # Add stores from ways to Redis
        for store in store_ways:
            # Add store to hash with store id as key and store and address as value if the store has the required tags
            if 'name' in store['tags'] and 'addr:street' in store['tags'] and 'addr:housenumber' in store[
                'tags'] and 'addr:city' in store['tags']:
                # Add store as hash
                r.geoadd('lidl_geo', (store['center']['lon'], store['center']['lat'], store['id']), nx=True)
                r.hset('lidl_location:' + str(store['id']),
                       mapping={'name': store['tags']['name'], 'street': store['tags']['addr:street'],
                                'housenumber': store['tags']['addr:housenumber'], 'city': store['tags']['addr:city']})
                # Add store to city index
                r.sadd('lidl_index:city:' + store['tags']['addr:city'].lower(), store['id'])
                included_stores += 1

        print(f"Processed {len(store_nodes) + len(store_ways)} Lidl locations.")
        print(f"Actually included {included_stores} stores.")

except FileNotFoundError as e:
    print(f"Error: Could not read file: {e}")
except json.JSONDecodeError as e:
    print(f"Error: Could not decode JSON: {e}")
except redis.RedisError as e:
    print(f"Error: Could not write to Redis: {e}")

# Close Redis connection
r.close()
