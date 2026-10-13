pub type Result<T> = std::result::Result<T, Error>;

/// Error type for environment variable and configuration handling.
#[derive(Debug)]
pub enum Error {
    EnvVarError {
        var: String,
        /// Path of the field mapped to the variable (e.g. `server.port`), or `None` for the variable named by `env_name_key`.
        field: Option<&'static str>,
        source: std::env::VarError,
    },
    DotEnvyError(dotenvy::Error),
    ValConversionError(String),
    MissingEnvName,
    InvalidEnvName,
}

impl From<dotenvy::Error> for Error {
    fn from(err: dotenvy::Error) -> Self {
        Error::DotEnvyError(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::EnvVarError { source, .. } => Some(source),
            Error::DotEnvyError(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EnvVarError { var, field, source } => {
                match source {
                    std::env::VarError::NotPresent => {
                        write!(fmt, "environment variable `{var}` not found")?
                    }
                    std::env::VarError::NotUnicode(value) => write!(
                        fmt,
                        "environment variable `{var}` is not valid unicode: {value:?}"
                    )?,
                }
                if let Some(field) = field {
                    write!(fmt, " (field `{field}`)")?;
                }
                Ok(())
            }
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

    fn env_var_error(field: Option<&'static str>, source: std::env::VarError) -> Error {
        Error::EnvVarError {
            var: "DATABASE_URL".to_string(),
            field,
            source,
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(
            env_var_error(None, std::env::VarError::NotPresent).to_string(),
            "environment variable `DATABASE_URL` not found"
        );
        assert_eq!(
            env_var_error(Some("db.url"), std::env::VarError::NotPresent).to_string(),
            "environment variable `DATABASE_URL` not found (field `db.url`)"
        );
        assert_eq!(
            env_var_error(Some("db.url"), std::env::VarError::NotUnicode("x".into())).to_string(),
            "environment variable `DATABASE_URL` is not valid unicode: \"x\" (field `db.url`)"
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
            env_var_error(None, std::env::VarError::NotPresent)
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
