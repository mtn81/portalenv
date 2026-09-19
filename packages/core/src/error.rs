pub type Result<T> = std::result::Result<T, Error>;

/// Error type for environment variable and configuration handling.
#[derive(Debug)]
pub enum Error {
    EnvValError(std::env::VarError),
    DotEnvyError(dotenvy::Error),
    ValConversionError(String),
    MissingEnvName,
    InvalidEnvName,
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::EnvValError(err)
    }
}

impl From<dotenvy::Error> for Error {
    fn from(err: dotenvy::Error) -> Self {
        Error::DotEnvyError(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::EnvValError(err) => Some(err),
            Error::DotEnvyError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EnvValError(err) => write!(fmt, "{}", err),
            Error::DotEnvyError(err) => write!(fmt, "{}", err),
            Error::ValConversionError(err) => write!(fmt, "{}", err),
            Error::MissingEnvName => write!(fmt, "Missing environment name"),
            Error::InvalidEnvName => write!(fmt, "Invalid environment name"),
        }
    }
}
