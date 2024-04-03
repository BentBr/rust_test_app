use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Counter {
    pub count: i64,
}

impl Counter {
    fn get_redis_url() -> String {
        env::var("REDIS_DSN").expect("REDIS_DSN not set")
    }

    pub fn save(self) -> Result<(), redis::RedisError> {
        let serialized = serde_json::to_vec(&self).unwrap();
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut connection = match client.get_connection() {
            Ok(connection) => connection,
            Err(error) => return Err(error),
        };

        match redis::cmd("SET")
            .arg("COUNTER_JSON")
            .arg(serialized)
            .query::<Vec<u8>>(&mut connection)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn load() -> Result<Counter, redis::RedisError> {
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error),
        };
        let mut con = match client.get_connection() {
            Ok(con) => con,
            Err(error) => return Err(error),
        };
        let byte_data: Vec<u8> = match redis::cmd("GET").arg("COUNTER_JSON").query(&mut con) {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        Ok(serde_json::from_slice(&byte_data).unwrap())
    }

    pub fn initialize_if_not_exists() -> Result<Counter, redis::RedisError> {
        let client = redis::Client::open(Counter::get_redis_url())?;
        let mut connection = client.get_connection()?;

        let exists: bool = redis::cmd("EXISTS")
            .arg("COUNTER_JSON")
            .query(&mut connection)?;
        if !exists {
            let initial_counter = Counter { count: 0 };
            initial_counter.save()?;
            Ok(initial_counter)
        } else {
            Counter::load()
        }
    }

    pub fn increment() -> Result<i64, redis::RedisError> {
        let client = redis::Client::open(Counter::get_redis_url())?;
        let mut connection = client.get_connection()?;

        let new_count: i64 = redis::cmd("INCR").arg("COUNTER").query(&mut connection)?;
        Ok(new_count)
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
