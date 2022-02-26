#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] ::std::io::Error),
    #[error("channel recv error")]
    Recv(#[from] ::crossbeam_channel::RecvError),
    #[error("from utf8 error")]
    FromUtf8(#[from] ::std::string::FromUtf8Error),
    #[error("we should stop listening on the tcp stream")]
    StopListening,
}

pub type Result<T> = std::result::Result<T, Error>;
