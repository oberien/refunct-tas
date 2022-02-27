#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tcp error")]
    TcpError(#[from] ::std::io::Error),
    #[error("we should stop listening on the tcp stream")]
    StopListening,
}

pub type Result<T> = std::result::Result<T, Error>;
