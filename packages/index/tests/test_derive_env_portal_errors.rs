#![allow(dead_code)]

use portalenv::*;
use serial_test::serial;

#[derive(Debug, PartialEq, EnvEnum)]
pub enum TestEnum {
    A,
    B,
}

#[test]
#[serial]
fn test_env_var_not_set() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: env_var::ERROR_TEST_NOT_SET,
    })]
    pub struct EnvConfig {
        pub hoge: String,
    }

    unsafe {
        std::env::remove_var("ERROR_TEST_NOT_SET");
    }
    let result = EnvConfig::from_env();
    assert!(matches!(
        result,
        Err(Error::EnvValError(std::env::VarError::NotPresent))
    ));
}

#[test]
#[serial]
fn test_env_var_conversion_failure() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: env_var::ERROR_TEST_NOT_A_NUMBER,
    })]
    pub struct EnvConfig {
        pub hoge: u32,
    }

    unsafe {
        std::env::set_var("ERROR_TEST_NOT_A_NUMBER", "abc");
    }
    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::ValConversionError(_))));
}

#[test]
#[serial]
fn test_env_match_without_env_name_key() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: env_match {
            "local" => "hoge-local-value",
        },
    })]
    pub struct EnvConfig {
        pub hoge: String,
    }

    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::MissingEnvName)));
}

#[test]
#[serial]
fn test_env_partial_match_without_env_name_key() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: env_partial_match {
            "local" => "hoge-local-value",
        },
    })]
    pub struct EnvConfig {
        pub hoge: Option<String>,
    }

    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::MissingEnvName)));
}

#[test]
#[serial]
fn test_env_name_key_not_set() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "ERROR_TEST_ENV_NAME",
        mapping = {
            hoge: "hoge-value",
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
    }

    unsafe {
        std::env::remove_var("ERROR_TEST_ENV_NAME");
    }
    let result = EnvConfig::from_env();
    assert!(matches!(
        result,
        Err(Error::EnvValError(std::env::VarError::NotPresent))
    ));
}

#[test]
#[serial]
fn test_env_include_propagates_error() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: "hoge-value",
        other: env_include<OtherEnvConfig>,
    })]
    pub struct EnvConfig {
        pub hoge: String,
        pub other: OtherEnvConfig,
    }

    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        foo: env_var::ERROR_TEST_INCLUDED_NOT_SET,
    })]
    pub struct OtherEnvConfig {
        pub foo: String,
    }

    unsafe {
        std::env::remove_var("ERROR_TEST_INCLUDED_NOT_SET");
    }
    let result = EnvConfig::from_env();
    assert!(matches!(
        result,
        Err(Error::EnvValError(std::env::VarError::NotPresent))
    ));
}

#[test]
#[serial]
fn test_env_enum_unknown_variant() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: "C",
    })]
    pub struct EnvConfig {
        pub hoge: TestEnum,
    }

    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::ValConversionError(_))));

    for value in ["C", "a", "OtherEnum::A"] {
        let result: Result<TestEnum> = EnvStrValue::new("test", value).convert();
        assert!(
            matches!(result, Err(Error::ValConversionError(_))),
            "{value} should not convert into TestEnum"
        );
    }
}
