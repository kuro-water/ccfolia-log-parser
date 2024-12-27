use thiserror::Error;

#[derive(Error, Debug)]
#[error("{}", string)]
pub struct ParseError {
    pub(crate) string: String,
}

// 参考：https://zenn.dev/shimopino/articles/understand-rust-error-handling#thiserror-%E3%82%AF%E3%83%AC%E3%83%BC%E3%83%88
#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] ParseError),
}
