/// 统一错误处理, 负责转换各种系统错误信息
///
///
///

#[derive(Debug)]
pub enum RedisError {
    IoError(std::io::Error),
    AddrParseError(std::net::AddrParseError),
}

impl std::error::Error for RedisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            RedisError::IoError(ref e) => Some(e),
            RedisError::AddrParseError(ref e) => Some(e),
        }
    }
}

///实现Display的trait，并实现fmt方法
impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RedisError::IoError(ref e) => e.fmt(f),
            RedisError::AddrParseError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RedisError {
    fn from(s: std::io::Error) -> Self {
        RedisError::IoError(s)
    }
}

/// 转换AddrParseError 错误到RedisError
impl From<std::net::AddrParseError> for RedisError {
    fn from(s: std::net::AddrParseError) -> Self {
        RedisError::AddrParseError(s)
    }
}
