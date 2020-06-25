use crate::redis_client::RedisClient;
use crate::redis_command::RedisCommand;
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

    pub fn set(&mut self, key: &str, val: &str) -> bool {
        self.cmd
            .add_array(3)
            .add_buik_string("SET")
            .add_buik_string(key)
            .add_buik_string(val);
        println!("befor write");
        match self.cmd.write() {
            Err(_) => {
                return false;
            }
            Ok(_) => {}
            //_ => {
            //},
        }
        println!("after write");
        self.cmd.check_status()
    }

    pub fn get(&mut self, key: &str) -> Result<String, RedisError> {
        self.cmd
            .add_array(2)
            .add_buik_string("GET")
            .add_buik_string(key);
        self.cmd.write()?;
        self.cmd.read_string()
    }
}
