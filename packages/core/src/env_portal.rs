use std::path::Path;

use crate::error::*;

pub trait EnvPortal {
    fn env_name_key() -> Option<&'static str>;
    fn dotenv_file() -> Option<&'static str>;

    fn from_env() -> Result<Self>
    where
        Self: Sized,
    {
        let env = Self::env_name_key()
            .map(|key| std::env::var(key))
            .transpose()?;

        // load from .env file if it exists
        if let Some(file) = Self::dotenv_file() {
            if Path::new(file).exists() {
                dotenvy::from_filename_override(file)?;
            }
            if let Some(ref env) = env {
                let file = &format!("{file}.{env}");
                if Path::new(file).exists() {
                    dotenvy::from_filename_override(file)?;
                }
            }
        }

        Self::from_env_with(env)
    }

    fn from_env_with(env: Option<String>) -> Result<Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
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
    fn test_try_from_env() {
        unsafe {
            std::env::set_var("TEST_ENV_KEY", "test_env");
        }
        let result = TestStruct::from_env().unwrap();
        assert_eq!(result.value, "test_env".to_string());
    }
}
