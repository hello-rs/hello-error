use thiserror::Error;
// 1. 实现 Debug 与 std::fmt::Display
// 2. 实现 std::error::Error 特质
//  - 没有子 Error 只需要 `impl std::error::Error for <YourError> {}`
//  - 有子 Error 需要实现 source() 返回 Some(<YourChildError>)
#[derive(Error, Debug)]
pub enum CustomError {
    #[error("number `{0}` is gt 200")]
    NumberGt200Error(usize),
    #[error("not found")]
    NotFoundError,
    #[error(transparent)]
    Network(NetworkError),
    // transparent
    // 将 source 和 display 方法直接转发到基础错误(io::Error),而不添加其他消息.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("time out")]
    TimeOut,
}

// impl From<std::io::Error> for CustomError {
//     fn from(e: std::io::Error) -> Self {
//         Self::IOError(e)
//     }
// }

// impl Error for CustomError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             CustomError::Network(e) => Some(e),
//             _ => None,
//         }
//     }
// }

// impl Display for CustomError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CustomError::NumberGt200Error(number) => write!(f, "number is {}, 期待小于200", number),
//             CustomError::NotFoundError => write!(f, "not found"),
//             CustomError::Network(sub) => write!(f, "{}", sub),
//             CustomError::IOError(sub) => write!(f, "{}", sub),
//         }
//     }
// }

// impl Error for NetworkError {}

// impl Display for NetworkError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             NetworkError::TimeOut => write!(f, "time out"),
//         }
//     }
// }
