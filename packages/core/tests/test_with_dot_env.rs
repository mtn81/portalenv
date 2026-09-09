use portalenv_core::*;

#[test]
fn test_with_dot_env() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
        another_value: String,
    }

    impl EnvPortal for TestStruct {
        fn env_name_key() -> Option<&'static str> {
            None
        }

        fn dotenv_file() -> Option<&'static str> {
            Some("tests/.env")
        }

        fn from_env_with(_env: Option<String>) -> Result<Self> {
            let value = std::env::var("VALUE").unwrap();
            let another_value = std::env::var("ANOTHER_VALUE").unwrap();
            Ok(TestStruct {
                value,
                another_value,
            })
        }
    }

    let result = TestStruct::from_env().unwrap();
    assert_eq!(result.value, "test-dotenv-value".to_string());
    assert_eq!(result.another_value, "test-another-value".to_string());
}

#[test]
fn test_with_env_key_and_dot_env() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
        another_value: String,
    }

    impl EnvPortal for TestStruct {
        fn env_name_key() -> Option<&'static str> {
            Some("TEST_ENV_KEY")
        }

        fn dotenv_file() -> Option<&'static str> {
            Some("tests/.env")
        }

        fn from_env_with(_env: Option<String>) -> Result<Self> {
            let value = std::env::var("VALUE").unwrap();
            let another_value = std::env::var("ANOTHER_VALUE").unwrap();
            Ok(TestStruct {
                value,
                another_value,
            })
        }
    }

    unsafe {
        std::env::set_var("TEST_ENV_KEY", "test");
    }
    let result = TestStruct::from_env().unwrap();
    assert_eq!(result.value, "test-dotenv-value-test".to_string());
    assert_eq!(result.another_value, "test-another-value".to_string());
}
