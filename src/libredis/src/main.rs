mod redis_client;
mod redis_command;
mod redis_error;
mod redis_result;
mod redis_string;

use redis_client::RedisClient;
use redis_string::RedisString;

fn main() {
    let addr = "127.0.0.1:6379";
    let conn_timeout = 0u64;
    let rw_timeout = 3u64;
    let mut stream = RedisClient::new(addr, &conn_timeout, &rw_timeout).unwrap();
    let key = "a";
    let mut rstr = RedisString::new(&mut stream);
    println!("timout ={}", conn_timeout);
    if rstr.set(&key, "whb") {
        println!("to get data");
        match rstr.get("a") {
            Ok(value) => {
                println!("main get {}={}", key, value);
            }
            Err(err) => {
                println!("err={:?}", err);
            }
        }
    }
}
