use crate::redis_client::RedisClient;
use crate::redis_command::{RedisCommand, AddBulkString};
///
///
/// redis string 操作
use crate::redis_error::RedisError;

pub struct RedisString<'a> {
    //RedisString 将声明周期传递给cmd, 参考<rustc --explain E0106>
    cmd: RedisCommand<'a>,
}

impl<'a> RedisString<'a> {
    pub fn new(cli: &'a mut RedisClient) -> RedisString {
        RedisString {
            cmd: RedisCommand::new(cli),
        }
    }

    /// 将字符串值 val 关联到 key
    /// @return true 操作是否成功，返回 false 表示出错或该 key 对象非字符串对象
    pub fn set(&mut self, key: &str, val: &str) -> bool {
        self.cmd
            .add_array(3)
            .add_bulk_string("SET")
            .add_bulk_string(key)
            .add_bulk_string(val).check_status()
    }

    /// 将val关联到 key, 并将 key 的生存时间设为 timeout (以秒为单位)
    /// 如果 key 已经存在， SETEX 命令将覆写旧值
    pub fn setex(&mut self, key: &str, val: &mut Vec<u8>, timeout: u32) -> bool {
            self.cmd
            .add_array(4)
            .add_bulk_string("SET")
            .add_bulk_string(key)
            .add_bulk_string(timeout)
            .add_bulk_string(val).check_status()
    }

    pub fn get(&mut self, key: &str) -> Result<String, RedisError> {
        self.cmd
            .add_array(2)
            .add_bulk_string("GET")
            .add_bulk_string(key);
        self.cmd.write()?;
        self.cmd.read_string()
    }
}
