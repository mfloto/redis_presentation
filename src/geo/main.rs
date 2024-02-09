use redis;
use redis::Connection;
use redis_demo::create_redis_connection;
use std::collections::{HashMap, HashSet};

/// Get all the stores in a city using the city index
fn get_stores_by_city(
    connection: &mut Connection,
    city_name: &str,
) -> redis::RedisResult<Vec<String>> {
    let store_ids = redis::cmd("SMEMBERS")
        .arg(format!("lidl_index:city:{}", city_name))
        .query(connection)?;
    Ok(store_ids)
}

/// Get the distance between two stores
fn get_distance_between_stores(
    connection: &mut Connection,
    store_id1: &str,
    store_id2: &str,
) -> redis::RedisResult<f64> {
    let distance: f64 = redis::cmd("GEODIST")
        .arg("lidl_loc")
        .arg(store_id1)
        .arg(store_id2)
        .arg("km")
        .query(connection)?;
    Ok(distance)
}

/// Calculate the distance between all stores in a city and store the distances in a HashMap
fn calculate_all_distances(
    connection: &mut Connection,
    store_ids: Vec<String>,
) -> redis::RedisResult<HashMap<String, f64>> {
    let mut distances = HashMap::new();
    let mut visited_stores = HashSet::new(); // Avoiding duplicate distances

    for store_id1 in &store_ids {
        visited_stores.insert(store_id1);
        for store_id2 in &store_ids {
            if store_id1 != store_id2 && !visited_stores.contains(store_id2) {
                let distance = get_distance_between_stores(connection, store_id1, store_id2)?;
                distances.insert(format!("{}<->{}", store_id1, store_id2), distance);
            }
        }
    }
    Ok(distances)
}

/// Store the distances in a sorted set in Redis
fn store_distances_to_sorted_set(
    connection: &mut Connection,
    distances: HashMap<String, f64>,
    city_name: &str,
) -> redis::RedisResult<()> {
    for (store_pair, distance) in distances {
        redis::cmd("ZADD")
            .arg(format!("lidl_distances:{}", city_name))
            .arg(distance)
            .arg(store_pair)
            .query(connection)?;
    }
    Ok(())
}

/// Get the average distance between all stores in a city
fn get_average_distance(connection: &mut Connection, city_name: &str) -> redis::RedisResult<f64> {
    let distances: Vec<(String, f64)> = redis::cmd("ZRANGE")
        .arg(format!("lidl_distances:{}", city_name))
        .arg(0)
        .arg(-1)
        .arg("WITHSCORES")
        .query(connection)?;
    let sum: f64 = distances.iter().map(|(_, distance)| distance).sum();
    Ok(sum / distances.len() as f64)
}

/// Get the maximum distance from all stores in a city
fn get_max_distance(connection: &mut Connection, city_name: &str) -> redis::RedisResult<f64> {
    let max_distance: (String, f64) = redis::cmd("ZRANGE")
        .arg(format!("lidl_distances:{}", city_name))
        .arg(-1)
        .arg(-1)
        .arg("WITHSCORES")
        .query(connection)?;
    Ok(max_distance.1)
}

/// Get the minimum distance between from all stores in a city
fn get_min_distance(connection: &mut Connection, city_name: &str) -> redis::RedisResult<f64> {
    let min_distance: (String, f64) = redis::cmd("ZRANGE")
        .arg(format!("lidl_distances:{}", city_name))
        .arg(0)
        .arg(0)
        .arg("WITHSCORES")
        .query(connection)?;
    Ok(min_distance.1)
}

fn main() {
    let mut connection = create_redis_connection();
    if let Ok(mannheim_stores) = get_stores_by_city(&mut connection, "mannheim") {
        println!("Stores in Mannheim: {:?}\n", mannheim_stores);
        // Calculate the distance between all stores in that city
        if let Ok(distances) = calculate_all_distances(&mut connection, mannheim_stores) {
            // Store the distances in a sorted set
            if let Ok(()) = store_distances_to_sorted_set(&mut connection, distances, "mannheim") {
                println!("Stored the distances in a sorted set\n");
            }

            // Average distance between all stores in Mannheim
            if let Ok(average_distance) = get_average_distance(&mut connection, "mannheim") {
                println!(
                    "Average distance between stores in Mannheim: {:.2} km\n",
                    average_distance
                );
            }

            // Maximum distance between all stores in Mannheim
            if let Ok(max_distance) = get_max_distance(&mut connection, "mannheim") {
                println!(
                    "Maximum distance between stores in Mannheim: {:.2} km\n",
                    max_distance
                );
            }

            // Minimum distance between all stores in Mannheim
            if let Ok(min_distance) = get_min_distance(&mut connection, "mannheim") {
                println!(
                    "Minimum distance between stores in Mannheim: {:.2} km\n",
                    min_distance
                );
            }
        }
    }
}
