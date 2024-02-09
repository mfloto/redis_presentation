pub fn create_redis_connection() -> redis::Connection {
    let client = redis::Client::open("redis://redis:6379/").unwrap();
    client.get_connection().unwrap()
}