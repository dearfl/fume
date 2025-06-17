use thiserror::Error;

#[cfg(feature = "async")]
use fume_async::Backend;

#[cfg(feature = "blocking")]
use fume_blocking::Backend;

#[derive(Debug, Error)]
pub enum Error<B>
where
    B: Backend,
{
    #[error("Backend Error: {0}")]
    BackendError(B::Error),
    #[error("Decode Error: {0}")]
    DecodeError(#[from] serde_json::Error),
}
