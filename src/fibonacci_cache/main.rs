use redis;
use redis_demo::create_redis_connection;

struct Parameters {
    n: u32,
    use_cache: bool,
}

fn get_parameters() -> Parameters {
    // Get from command line the number of fibonacci numbers to calculate
    let n: u32 = std::env::args()
        .nth(1)
        .expect("Please provide the number of fibonacci numbers to calculate")
        .parse()
        .expect("Please provide a valid number");
    // Get from command line if the redis cache should be used
    let use_cache: bool = std::env::args()
        .nth(2)
        .expect("Please provide if the cache should be used")
        .parse()
        .expect("Please provide a valid boolean");
    Parameters { n, use_cache }
}

fn fibonacci_no_cache(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    println!("Calculating fibonacci number {}", n);
    fibonacci_no_cache(n - 1) + fibonacci_no_cache(n - 2)
}

fn fibonacci_with_cache(n: u32, connection: &mut redis::Connection) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    // Check if the number is already in the cache
    let cached: Option<u32> = redis::cmd("GET")
        .arg(format!("fibonacci:{}", n))
        .query(connection)
        .expect("Failed to get the value from the cache");
    if let Some(cached_value) = cached {
        println!("Found fibonacci number {} in the cache", n);
        return cached_value;
    }
    println!("Calculating fibonacci number {}", n);
    let result = fibonacci_with_cache(n - 1, connection) + fibonacci_with_cache(n - 2, connection);
    // Store the result in the cache
    let _: () = redis::cmd("SET")
        .arg(format!("fibonacci:{}", n))
        .arg(result)
        .execute(connection);
    result
}

fn main() {
    // Get the parameters from the command line
    let parameters = get_parameters();
    // Create a redis connection
    let mut connection = create_redis_connection();
    // Calculate the fibonacci numbers
    let start = std::time::Instant::now();
    let fib = if parameters.use_cache {
        fibonacci_with_cache(parameters.n, &mut connection)
    } else {
        fibonacci_no_cache(parameters.n)
    };
    let elapsed = start.elapsed();
    println!(
        "The {}th fibonacci number is {} and was calculated in {} ms",
        parameters.n,
        fib,
        elapsed.as_millis()
    );
}
