//! Error types of the crate.

use alloc::{borrow::Cow, string::String};
#[cfg(feature = "std")]
use std::io;
use thiserror::Error;

/// Spezialized result type for oci spec operations. It is
/// used for any operation that might produce an error. This
/// typedef is generally used to avoid writing out
/// [OciSpecError] directly and is otherwise a direct mapping
/// to [Result](std::result::Result).
pub type Result<T> = core::result::Result<T, OciSpecError>;

/// Error type for oci spec errors.
#[derive(Error, Debug)]
pub enum OciSpecError {
    /// Will be returned if an error occurs that cannot
    /// be mapped to a more specialized error variant.
    #[error("{0}")]
    Other(String),

    /// Will be returned when an error happens during
    /// io operations.
    #[error("io operation failed")]
    #[cfg(feature = "std")]
    Io(#[from] io::Error),

    /// Will be returned when an error happens during
    /// serialization or deserialization.
    #[error("serde failed")]
    SerDe(#[from] serde_json::Error),

    /// Builder specific errors.
    #[cfg(feature = "std")]
    #[error("uninitialized field")]
    Builder(#[from] derive_builder::UninitializedFieldError),
    /// Builder specific errors.
    #[cfg(not(feature = "std"))]
    #[error("uninitialized field")]
    Builder(derive_builder::UninitializedFieldError),
}

pub(crate) fn oci_error<'a, M>(message: M) -> OciSpecError
where
    M: Into<Cow<'a, str>>,
{
    let message = message.into();
    match message {
        Cow::Borrowed(s) => OciSpecError::Other(String::from(s)),
        Cow::Owned(s) => OciSpecError::Other(s),
    }
}

#[cfg(not(feature = "std"))]
impl From<derive_builder::UninitializedFieldError> for OciSpecError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        OciSpecError::Builder(error)
    }
}
