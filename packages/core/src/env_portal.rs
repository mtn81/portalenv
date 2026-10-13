use std::path::Path;

use crate::error::*;

/// A trait for types that can be constructed from configuration values and environment variables (optionally loaded from a .env file).
pub trait EnvPortal {
    fn env_name_key() -> Option<&'static str>;
    fn dotenv_file() -> Option<&'static str>;

    fn from_env() -> Result<Self>
    where
        Self: Sized,
    {
        let base_file = Self::dotenv_file().filter(|file| Path::new(file).exists());

        let env = Self::env_name_key()
            .map(|key| env_name(key, base_file))
            .transpose()?;

        // dotenvy::from_filename never overrides variables that are already set, so loading
        // the more specific file first gives: process env > `.env.{env}` > `.env`.
        if let Some(file) = Self::dotenv_file()
            && let Some(ref env) = env
        {
            let file = &format!("{file}.{env}");
            if Path::new(file).exists() {
                dotenvy::from_filename(file)?;
            }
        }

        if let Some(file) = base_file {
            dotenvy::from_filename(file)?;
        }

        Self::from_env_with(env)
    }

    fn from_env_with(env: Option<String>) -> Result<Self>
    where
        Self: Sized;
}

fn env_name(key: &str, base_file: Option<&str>) -> Result<String> {
    let error = |source| Error::EnvVarError {
        var: key.to_string(),
        field: None,
        source,
    };

    match std::env::var(key) {
        Err(std::env::VarError::NotPresent) => {
            if let Some(file) = base_file {
                for item in dotenvy::from_filename_iter(file)? {
                    let (name, value) = item?;
                    if name == key {
                        return Ok(value);
                    }
                }
            }
            Err(error(std::env::VarError::NotPresent))
        }
        result => result.map_err(error),
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
    }

    impl EnvPortal for TestStruct {
        fn env_name_key() -> Option<&'static str> {
            Some("TEST_ENV_KEY")
        }

        fn dotenv_file() -> Option<&'static str> {
            Some(".env")
        }

        fn from_env_with(env: Option<String>) -> Result<Self> {
            let value = env.unwrap_or_else(|| "default_value".to_string());
            Ok(TestStruct { value })
        }
    }

    #[test]
    #[serial]
    fn test_try_from_env() {
        unsafe {
            std::env::set_var("TEST_ENV_KEY", "test_env");
        }
        let result = TestStruct::from_env().unwrap();
        assert_eq!(result.value, "test_env".to_string());
    }
}
