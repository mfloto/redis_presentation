use redis;
use redis::geo::RadiusSearchResult;
use redis::FromRedisValue;
use redis_demo::create_redis_connection;

/// Struct to hold the parameters put in by the user
struct Parameters {
    latitude: f64,
    longitude: f64,
    radius: f64,
}

/// Struct to hold the store information
struct Store {
    street: String,
    house_number: String,
    city: String,
}

/// Implement the trait FromRedisValue for the Store struct to use it as a query result
impl FromRedisValue for Store {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Store> {
        match *v {
            redis::Value::Bulk(ref items) => {
                let street = redis::from_redis_value(&items[0])?;
                let house_number = redis::from_redis_value(&items[1])?;
                let city = redis::from_redis_value(&items[2])?;
                Ok(Store {
                    street,
                    house_number,
                    city,
                })
            }
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid type",
            ))),
        }
    }
}

/// Get the parameters from the command line
fn get_parameters() -> Parameters {
    // Get from command line the latitude, longitude and radius
    let latitude: f64 = std::env::args()
        .nth(1)
        .expect("Please provide the latitude")
        .parse()
        .expect("Please provide a valid number");
    let longitude: f64 = std::env::args()
        .nth(2)
        .expect("Please provide the longitude")
        .parse()
        .expect("Please provide a valid number");
    let radius: f64 = std::env::args()
        .nth(3)
        .expect("Please provide the radius in km")
        .parse()
        .expect("Please provide a valid number");
    Parameters {
        latitude,
        longitude,
        radius,
    }
}

/// Get all the stores in a radius using the city index
fn get_stores_in_radius(
    connection: &mut redis::Connection,
    parameters: &Parameters,
) -> redis::RedisResult<Vec<RadiusSearchResult>> {
    let store_ids: Vec<RadiusSearchResult> = redis::cmd("GEORADIUS")
        .arg("lidl_geo")
        .arg(parameters.longitude)
        .arg(parameters.latitude)
        .arg(parameters.radius)
        .arg("KM")
        .arg("WITHDIST")
        .query(connection)?;
    Ok(store_ids)
}

/// Get the store information (address only) by its id
fn get_store_by_id(
    connection: &mut redis::Connection,
    store_id: &str,
) -> redis::RedisResult<Store> {
    let store: Store = redis::cmd("HMGET")
        .arg(format!("lidl_location:{}", store_id))
        .arg("street")
        .arg("housenumber")
        .arg("city")
        .query(connection)?;
    Ok(store)
}

/// Display the store information
fn pretty_print_store(store: Store, distance: f64) {
    println!(
        "Lidl found in {} km: {} {}, {}",
        distance, store.street, store.house_number, store.city
    );
}

fn main() {
    let parameters = get_parameters();
    let mut connection = create_redis_connection();

    let store_ids = get_stores_in_radius(&mut connection, &parameters);
    println!(
        "Found {} stores in a radius of {} km\n",
        store_ids.as_ref().unwrap().len(),
        parameters.radius
    );

    for store_result in store_ids.unwrap() {
        let store = get_store_by_id(&mut connection, &store_result.name);
        pretty_print_store(store.unwrap(), store_result.dist.unwrap());
    }
}
