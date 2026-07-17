//! Error type for the herdr socket transport.

use std::fmt;

/// Anything that can go wrong talking to a herdr server.
#[derive(Debug)]
pub enum Error {
    /// `$HERDR_SOCKET_PATH` was not set when [`Connection::from_env`] was used.
    ///
    /// [`Connection::from_env`]: crate::Connection::from_env
    NoSocketPath,
    /// Underlying socket I/O failed.
    Io(std::io::Error),
    /// A request frame could not be serialized, or a response could not be
    /// deserialized into the expected type.
    Json(serde_json::Error),
    /// herdr answered the request with an error response.
    Herdr { code: String, message: String },
    /// The server closed the connection before answering.
    Closed,
    /// The server's `protocol` version differs from the one these generated
    /// types were built against ([`crate::PROTOCOL`]). Continuing would risk
    /// silently mis-decoding responses, so the client refuses.
    ProtocolMismatch { client: u32, server: u32 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoSocketPath => write!(f, "$HERDR_SOCKET_PATH is not set"),
            Error::Io(e) => write!(f, "herdr socket I/O error: {e}"),
            Error::Json(e) => write!(f, "herdr message (de)serialization error: {e}"),
            Error::Herdr { code, message } => write!(f, "herdr error [{code}]: {message}"),
            Error::Closed => write!(f, "herdr closed the connection before responding"),
            Error::ProtocolMismatch { client, server } => write!(
                f,
                "herdr protocol mismatch: client generated against {client}, server speaks {server}"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

/// Convenience alias for results from this crate.
pub type Result<T> = std::result::Result<T, Error>;
