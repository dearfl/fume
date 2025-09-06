use thiserror::Error;

use fume_backend::Backend;

/// Unified error definition, different backend will
/// return different error type.
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
