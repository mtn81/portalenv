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

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use super::*;

    #[test]
    fn test_display() {
        let var_error = std::env::VarError::NotPresent;
        assert_eq!(
            Error::from(var_error.clone()).to_string(),
            var_error.to_string()
        );

        let dotenvy_error = dotenvy::Error::LineParse("x".to_string(), 1);
        let expected = dotenvy_error.to_string();
        assert_eq!(Error::from(dotenvy_error).to_string(), expected);

        assert_eq!(
            Error::ValConversionError("message".to_string()).to_string(),
            "message"
        );
        assert_eq!(
            Error::MissingEnvName.to_string(),
            "Missing environment name"
        );
        assert_eq!(
            Error::InvalidEnvName.to_string(),
            "Invalid environment name"
        );
    }

    #[test]
    fn test_source() {
        assert!(
            Error::from(std::env::VarError::NotPresent)
                .source()
                .is_some()
        );
        assert!(
            Error::from(dotenvy::Error::LineParse("x".to_string(), 1))
                .source()
                .is_some()
        );
        assert!(
            Error::ValConversionError("message".to_string())
                .source()
                .is_none()
        );
        assert!(Error::MissingEnvName.source().is_none());
        assert!(Error::InvalidEnvName.source().is_none());
    }
}
