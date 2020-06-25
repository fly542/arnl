use crate::redis_client::RedisClient;
///
///
/// redis 结果分析
///
///
//use std::str;
use std::io::prelude::*;
use std::io::BufReader;

pub enum RedisResult {
    RString(String),           // 简单单行字符串
    RError(String),            // 错误结果
    RInt(i64),                 // 整形结果
    RBData(Vec<u8>),           // 单二进制安全字符串
    RArray(u32, Vec<Vec<u8>>), // 数组
}

impl RedisResult {
    /// 读取一组二进制数据, 读取一行长度信息，再读取一组数据
    fn read_group_rbdata<R: BufRead>(reader: &mut R) -> Option<Vec<u8>> {
        let mut buf: String = String::new();
        match reader.read_line(&mut buf) {
            Ok(_) => {
                // 不需要返回的长度
                match RedisResult::parse_type(buf) {
                    RedisResult::RBData(mut data) => match reader.read_exact(data.as_mut_slice()) {
                        Ok(()) => Some(data),
                        _ => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// 解析结果类型
    fn parse_type(data: String) -> RedisResult {
        let line = data.trim_end_matches("\r\n");
        println!("line={}", line);
        match line.as_bytes()[0] {
            b'+' => {
                return RedisResult::RString(line[1..].to_string());
            }
            b'-' => {
                return RedisResult::RError(line[1..].to_string());
            }
            b':' => match line[1..].parse::<i64>() {
                Ok(val) => RedisResult::RInt(val),
                Err(e) => {
                    println!("int = {}", e.to_string());
                    return RedisResult::RError(format!(
                        "{} parse to int failed, err={}",
                        line[1..].to_string(),
                        e.to_string()
                    ));
                }
            },
            b'$' => {
                match line[1..].parse::<usize>() {
                    Ok(val) => {
                        // 注意， 此处不能使用Vec::new()或Vec::with_capcacity(val)
                        return RedisResult::RBData(vec![0; val]);
                    }
                    Err(e) => {
                        return RedisResult::RError(format!(
                            "{} parse to int failed, err={}",
                            line[1..].to_string(),
                            e.to_string()
                        ));
                    }
                };
            }
            b'*' => {
                match line[1..].parse::<u32>() {
                    Ok(val) => return RedisResult::RArray(val, Vec::new()),
                    Err(e) => {
                        return RedisResult::RError(format!(
                            "{} parse to int failed, err={}",
                            line[1..].to_string(),
                            e.to_string()
                        ));
                    }
                };
            }
            _ => {
                return RedisResult::RError(format!("unknow error line={}", line.to_string()));
            }
        }
    }

    pub fn parse_result(cli: &mut RedisClient) -> RedisResult {
        let mut reader = BufReader::new(&cli.stream);
        let mut buf: String = String::new();
        match reader.read_line(&mut buf) {
            Ok(_) => {
                // 不需要返回的长度
                let mut ret = RedisResult::parse_type(buf);
                match ret {
                    RedisResult::RBData(ref mut data) => {
                        match reader.read_exact(data.as_mut_slice()) {
                            Ok(_) => ret,
                            Err(e) => RedisResult::RError(format!(
                                "read_line error, err={}",
                                e.to_string()
                            )),
                        }
                    }
                    RedisResult::RArray(len, ref mut dt) => {
                        for _seq in 0..len {
                            RedisResult::read_group_rbdata(&mut reader).map(|v| dt.push(v));
                            println!("read data len={}", len);
                        }
                        ret
                    }
                    _ => ret,
                }
            }
            Err(e) => {
                let s = format!("read_line error, err={}", e.to_string());
                RedisResult::RError(s)
            }
        }
    }
}
