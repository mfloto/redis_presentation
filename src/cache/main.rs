use redis;
use redis_demo::create_redis_connection;
use std::{thread, time};

struct DeepThought {
    connection: redis::Connection,
}

impl DeepThought {
    pub fn new() -> DeepThought {
        DeepThought {
            connection: create_redis_connection(),
        }
    }

    fn get_cached_answer(&mut self) -> redis::RedisResult<i32> {
        let answer: i32 = redis::cmd("GET") // Has the complexity of O(1)
            .arg("answer")
            .query(&mut self.connection)?;
        Ok(answer)
    }

    fn cache_answer(&mut self, answer: i32) {
        let _ = redis::cmd("SET") // Has the complexity of O(1)
            .arg("answer")
            .arg(answer)
            .arg("EX") // This sets the lifetime of the key to 5 seconds
            .arg(5)
            .execute(&mut self.connection);
    }

    pub fn compute(&mut self) {
        // Check if we already have the answer in the cache
        if let Ok(answer) = self.get_cached_answer() {
            println!("Found the answer in the cache! It is {}", answer);
            return;
        }

        // Cache does not include the answer
        println!("The answer isn't cached; let's compute it");
        println!("Computing the answer to the ultimate question...");
        // Pretend that this is a very expensive computation
        thread::sleep(time::Duration::from_secs(2));
        let answer = 42;
        // Store the answer in the cache
        self.cache_answer(answer);
    }
}

fn main() {
    println!("This is how you can use redis as a cache!");

    let mut deep_thought = DeepThought::new();
    deep_thought.compute(); // Answer is not cached
    deep_thought.compute(); // Answer is cached
    deep_thought.compute(); // Answer is still cached
    deep_thought.compute(); // Answer is still cached
    thread::sleep(time::Duration::from_secs(5));
    deep_thought.compute(); // Answer is not cached anymore
}
