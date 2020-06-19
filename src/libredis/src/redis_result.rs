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
    RBString(u32, String),      // 单二进制安全字符串
    RArray(u32, Vec<String>),   // 数组
}

impl RedisResult {
    /// 解析结果类型
    fn parse_type(data: String) -> RedisResult{
        let line = data.trim_end_matches("\r\n");
        match line.as_bytes()[0] {
            b'+' => {
                return RedisResult::RString(line[1..].to_string());
            },
            b'-' => {
                return RedisResult::RError(line[1..].to_string());
            }
            b':' => {
                match line[1..].parse::<i64>() {
                    Ok(val) => return RedisResult::RInt(val),
                    Err(e) => {
                        return RedisResult::RError(
                            format!("{} parse to int failed, err={}",
                                line[1..].to_string(), e.to_string()));
                    },
                };
            }
            b'$' => {
                match line[1..].parse::<u32>() {
                    Ok(val) => return RedisResult::RBString(val, String::new()),
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
        let ret: RedisResult;
        match reader.read_line(&mut buf) {
            Ok(_) => {
                        println!("buf={},", buf);
                ret = RedisResult::parse_type(buf);
                //match ret {
                //    RedisResult::RString(ref data) => {
                //        println!("ln={}, buf={},", s, data);
                //    },
                //    _ => {},
                //}
            },
            Err(e) => {
                let s = format!("read_line error, err={}", e.to_string());
                ret = RedisResult::RError(s);
            },
        }
        ret
    }

}
