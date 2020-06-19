use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use crate::redis_error::RedisError;

#[allow(dead_code)]
pub struct RedisClient {
    pub stream: TcpStream,
    rw_timeout: u64,
}

impl RedisClient {
    /// 根据连接地址，连接超时时间，和读写超时超时时间创建redis连接
    pub fn new(addr: &str, conn_timeout: &u64, rw_timeout: &u64) -> Result<RedisClient, RedisError> {
        if *conn_timeout > 0 {
            let server: SocketAddr = addr.parse()?;
            let stream = TcpStream::connect_timeout(&server, Duration::from_secs(*conn_timeout))?;
            Ok(RedisClient {
                stream,
                rw_timeout: *rw_timeout,
            })
        } else {
            let stream = TcpStream::connect(addr)?;
            Ok(RedisClient {
                stream: stream,
                rw_timeout: *rw_timeout,
            })
        }
    }

    //pub fn set(&mut self, key:&str, val:&str) {
    //    let mut cmd = RedisCommand::new();
    //    cmd.write_arrs(3)
    //}
}
