use thiserror::Error;

// 参考：https://zenn.dev/shimopino/articles/understand-rust-error-handling#thiserror-%E3%82%AF%E3%83%AC%E3%83%BC%E3%83%88
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Anyhow Error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
