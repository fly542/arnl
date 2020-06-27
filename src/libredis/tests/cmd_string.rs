use redis_client::RedisClient;
use redis_string::RedisString;

#[cfg(test)]
mod tests {
    #[test]
    fn base_string() {
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

        if rstr.setex("whb2", &mut "abc".to_string().into_bytes(), 10) {
            println!("success set whb2=abc timeout=10s");
        }

        println!("setnx dup={}", rstr.setnx("whb2", "abc"));
    }
}
