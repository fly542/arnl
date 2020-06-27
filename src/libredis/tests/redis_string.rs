use redis::redis_client::RedisClient;
use redis::redis_string::RedisString;

#[test]
fn base_string() {
    let addr = "127.0.0.1:6379";
    let conn_timeout = 0u64;
    let rw_timeout = 3u64;
    let mut stream = RedisClient::new(addr, &conn_timeout, &rw_timeout).unwrap();
    let key = "a";
    let value = "whb";
    let mut rstr = RedisString::new(&mut stream);
    assert_eq!(true, rstr.set(&key, value));
    assert_eq!(value.to_string(), rstr.get(key).unwrap());

    assert_eq!(
        true,
        rstr.setex("whb2", &mut "abc".to_string().into_bytes(), 10)
    );
    assert_eq!(0, rstr.setnx("whb2", "abc"));
}
