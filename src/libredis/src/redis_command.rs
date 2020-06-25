use std::io::prelude::*;
///
///
/// redis相关命令基础类
/// redis 相关协议解析参考
///     https://redis.io/topics/protocol
///     http://www.redis.cn/topics/protocol.html
///
//use std::str;
//use std::io::BufReader;
use crate::redis_client::RedisClient;
use crate::redis_error::RedisError;
use crate::redis_result::RedisResult;

pub struct RedisCommand<'a> {
    cmd_str: Vec<u8>,
    conn: &'a mut RedisClient,
}

impl<'a> RedisCommand<'a> {
    /// 构造空的RedisCommand
    pub fn new(cli: &'a mut RedisClient) -> RedisCommand {
        RedisCommand {
            cmd_str: Vec::new(),
            conn: cli,
        }
    }
    /// 添加二进制字节, 会移动 `other` 中的内容到 `Self` 中,
    /// 添加后 `other` 将为空
    pub fn add_bytes(&mut self, other: &mut Vec<u8>) -> &mut Self {
        self.cmd_str.append(other);
        self
    }

    /// 添加单个字符,
    /// 不使用s as u8, 因为考虑char为4个字节，单个u8可能无法表示
    pub fn add_char(&mut self, s: char) -> &mut Self {
        self.add_str(s.to_string())
    }

    /// 添加字符串
    pub fn add_str(&mut self, s: String) -> &mut Self {
        self.add_bytes(&mut s.into_bytes())
    }

    /// 添加回车换行
    pub fn add_crnl(&mut self) -> &mut Self {
        self.add_str("\r\n".to_string())
    }

    /// 添加数值
    pub fn add_usize(&mut self, n: usize) -> &mut Self {
        self.add_str(n.to_string())
    }

    /// 添加要发送的字段个数
    pub fn add_array(&mut self, n: usize) -> &mut Self {
        self.add_char('*').add_usize(n).add_crnl()
    }

    pub fn add_bulk_string(&mut self, s:&mut Vec<u8>) -> &mut Self {
        if s.is_empty() {
            self.add_str("$-1".to_string()).add_crnl()
        } else {
            self.add_char('$')
                .add_usize(s.len())
                .add_crnl()
                .add_bytes(s)
                .add_crnl()
        }
    }

    /// 返回命令内容字节信息
    //pub fn as_bytes(&self) ->&[u8] {
    //    self.cmd_str.as_bytes()
    //}

    ///写入数据到服务端
    pub fn write(&mut self) -> Result<(), RedisError> {
        let ret = self.conn.stream.write_all(self.cmd_str.as_slice());
        self.cmd_str.clear();
        if let Err(e) = ret {
            Err(RedisError::IoError(e))
        } else {
            Ok(())
        }
    }

    pub fn read_string(&mut self) -> Result<String, RedisError> {
        match RedisResult::parse_result(&mut self.conn) {
            RedisResult::RString(ret) => return Ok(ret),
            RedisResult::RError(ret) => return Err(RedisError::Info(ret.to_string())),
            RedisResult::RBData(ret) => match String::from_utf8(ret) {
                Ok(ret) => {
                    println!("data={}", ret);
                    return Ok(ret);
                }
                Err(e) => return Err(RedisError::Info(e.to_string())),
            },
            _ => return Err(RedisError::Info("not string".to_string())),
            //TODO 错误
        }

        ////read函数传递参数为mut 切片 fn read(&mut self, buf: &mut [u8])
        //let mut buffer = [0; 512];
        //self.conn.stream.read(&mut buffer).unwrap();
        //let response = str::from_utf8(&buffer).unwrap();
        //Ok(response.to_string())
    }

    pub fn check_status(&mut self) -> bool {
        match RedisResult::parse_result(&mut self.conn) {
            RedisResult::RString(_) => return true,
            _ => return false,
        };
        //let mut reader = BufReader::new(&self.conn.stream);
        //let mut buf: String = String::new();
        //match reader.read_line(&mut buf) {
        //    Ok(s) => {
        //        println!("ln={}, buf={}", s, ret);
        //    }
        //    Err(e) => {
        //        println!("err={}", e);
        //        return false;
        //    }
        //};
        //true

        //let mut reader = BufReader::new(&self.conn.stream);
        //let mut buf: Vec<u8> = Vec::new();
        //match reader.read_until(b'\n', &mut buf) {
        //    Ok(s) => {
        //        match str::from_utf8(&buf) {
        //            Ok(st) => {
        //                println!("status len={} {}",s, st);
        //            },
        //            Err(e) => {
        //                println!("reader err={}", e);
        //            },
        //        }
        //        return true;
        //    },
        //    Err(e) => {
        //        println!(" reader err={}", e);
        //        return false;
        //    },
        //}
        //true

        //let mut buf = [0; 512];
        //println!("befor read, {}", line!());
        //self.conn.stream.read(&mut buf[..]).unwrap();
        //println!("after read {}, buf.len={}", line!(), buf.len());
        //true

        //match self.read_string() {
        //    Ok(data) => {
        //        println!("read data={}", data);
        //        true
        //    },
        //    Err(_) => false,
        //}
    }
}
