#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("can't connect to rtil")]
    CantConnectToRtil,
}

pub type Result<T> = std::result::Result<T, Error>;
