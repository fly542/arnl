///
///
/// redis 结果分析
///
///

//use std::str;
use std::io::prelude::*;
use std::io::BufReader;
use crate::redis_client::RedisClient;

pub enum RedisResult {
    RString(String),            // 简单单行字符串
    RError(String),             // 错误结果
    RInt(i64),                  // 整形结果
    RBString(Vec<u8>),      // 单二进制安全字符串
    RArray(u32, Vec<Vec<u8>>),   // 数组
}

impl RedisResult {
    /// 解析结果类型
    fn parse_type(data: String) -> RedisResult{
        let line = data.trim_end_matches("\r\n");
        println!("line={}", line);
        match line.as_bytes()[0] {
            b'+' => {
                return RedisResult::RString(line[1..].to_string());
            },
            b'-' => {
                return RedisResult::RError(line[1..].to_string());
            }
            b':' => {
                match line[1..].parse::<i64>() {
                    Ok(val) => RedisResult::RInt(val),
                    Err(e) => {
                        println!("int = {}", e.to_string());
                        return RedisResult::RError(
                            format!("{} parse to int failed, err={}",
                                line[1..].to_string(), e.to_string()));
                    },
                }
            }
            b'$' => {
                match line[1..].parse::<usize>() {
                    Ok(val) => {
                        // 注意， 此处不能使用Vec::new()或Vec::with_capcacity(val)
                        return RedisResult::RBString(vec![0; val]);
                    },
                    Err(e) => {
                        return RedisResult::RError(
                            format!("{} parse to int failed, err={}",
                                line[1..].to_string(), e.to_string()));
                    },
                };
            }
            b'*' => {
                match line[1..].parse::<u32>() {
                    Ok(val) => return RedisResult::RArray(val, Vec::new()),
                    Err(e) => {
                        return RedisResult::RError(
                            format!("{} parse to int failed, err={}",
                                line[1..].to_string(), e.to_string()));
                    },
                };
            }
            _ => {
                return RedisResult::RError(format!("unknow error line={}",
                        line.to_string()));
            }
        }
    }

    pub fn parse_result(cli: &mut RedisClient) -> RedisResult{
        let mut reader = BufReader::new(&cli.stream);
        let mut buf: String = String::new();
        match reader.read_line(&mut buf) {
            Ok(_) => { // 不需要返回的长度
                let mut ret = RedisResult::parse_type(buf);
                match ret {
                    RedisResult::RBString(ref mut data) => {
                        match reader.read_exact(data.as_mut_slice()) {
                            Ok(()) => {
                                println!("will read = {}",
                                String::from_utf8(data.to_vec()).expect("aaa"));
                                return ret;
                            }
                            Err(e) => return RedisResult::RError(
                                format!("read_line error, err={}", e.to_string())),
                        }
                    }
                    RedisResult::RArray(len, ref dt) => {
                        println!("read data len={}", len);
                        return ret;
                    }
                    _ => ret,
                }
            },
            Err(e) => {
                let s = format!("read_line error, err={}", e.to_string());
                RedisResult::RError(s)
            },
        }
    }

}
