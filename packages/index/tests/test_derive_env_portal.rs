#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};

use portalenv::*;
use serial_test::serial;

#[derive(Debug, PartialEq, Eq, Hash, EnvEnum)]
pub enum TestEnum {
    A,
    B,
}

#[test]
fn test_derive_env_portal_for_empty() {
    #[derive(EnvPortal)]
    pub struct EnvConfig {}

    let _ = EnvConfig::from_env().unwrap();

    struct Hoge {
        aaa: String,
        bbb: String,
    }

    let h1 = Hoge {
        aaa: "aaa-value".to_string(),
        bbb: "bbb-value".to_string(),
    };
    let h2 = Hoge {
        aaa: "aaa-value".to_string(),
        bbb: "bbb-value".to_string(),
    };
    Hoge {
        aaa: "aaa-value".to_string(),
        ..h1
    };
}

#[test]
fn test_derive_env_portal_for_expr() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        foo: 10,
        foo2: 1 + 2,
        bar: true,
        bar2: true && false,
        baz: 3.14,
        baz2: if false { 3.14 } else { 0.0 },
        piyo: TestEnum::B,
    })]
    pub struct EnvConfig {
        pub foo: u32,
        pub foo2: u32,
        pub bar: bool,
        pub bar2: bool,
        pub baz: f64,
        pub baz2: f64,
        pub piyo: TestEnum,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.foo, 10);
    assert_eq!(config.foo2, 3);
    assert_eq!(config.bar, true);
    assert_eq!(config.bar2, false);
    assert_eq!(config.baz, 3.14);
    assert_eq!(config.baz2, 0.0);
    assert_eq!(config.piyo, TestEnum::B);
}

#[test]
fn test_derive_env_portal_for_env_str_value() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: "hoge-value",
        foo: "10",
        bar: "true",
        baz: "3.14",
        piyo: "B",
    })]
    pub struct EnvConfig {
        pub hoge: String,
        pub foo: u32,
        pub bar: bool,
        pub baz: f64,
        pub piyo: TestEnum,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-value".to_string());
    assert_eq!(config.foo, 10);
    assert_eq!(config.bar, true);
    assert_eq!(config.baz, 3.14);
    assert_eq!(config.piyo, TestEnum::B);
}

#[test]
fn test_derive_env_portal_for_nested() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: "hoge-value",
        foo: Foo {
            bar: "bar-value",
            baz: 3.14,
            piyo: TestEnum::B,
        },
    })]
    pub struct EnvConfig {
        pub hoge: String,
        pub foo: Foo,
    }
    pub struct Foo {
        pub bar: String,
        pub baz: f64,
        pub piyo: TestEnum,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-value".to_string());
    assert_eq!(config.foo.bar, "bar-value".to_string());
    assert_eq!(config.foo.baz, 3.14);
    assert_eq!(config.foo.piyo, TestEnum::B);
}

#[test]
fn test_derive_env_portal_for_complex_data() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: ["hoge1", "hoge2"],
        hoge2: [1, 3, 1],
        hoge3: r#"[ hoge1 , hoge2 ]"#,
        hoge4: ("hoge1", 1),
        bar: {
            bar1: 1,
            bar2: 2
        }
        baz: { A: "a", B: "b" },
        piyo: {
            "A": "a",
            "B": "b"
        },
        qux: r#"{ A: A, "TestEnum::B" : TestEnum::B }"#,
    })]
    pub struct EnvConfig {
        pub hoge: Vec<String>,
        pub hoge2: HashSet<u32>,
        pub hoge3: HashSet<String>,
        pub hoge4: (String, u32),
        pub bar: HashMap<String, u32>,
        pub baz: HashMap<TestEnum, String>,
        pub piyo: HashMap<TestEnum, String>,
        pub qux: HashMap<TestEnum, TestEnum>,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, vec!["hoge1".to_string(), "hoge2".to_string()]);
    assert_eq!(config.hoge2, HashSet::from([1, 3]));
    assert_eq!(
        config.hoge3,
        HashSet::from(["hoge1".to_string(), "hoge2".to_string()])
    );
    assert_eq!(config.hoge4, ("hoge1".to_string(), 1));
    assert_eq!(
        config.bar,
        HashMap::from([("bar1".to_string(), 1), ("bar2".to_string(), 2),])
    );
    assert_eq!(
        config.baz,
        HashMap::from([
            (TestEnum::A, "a".to_string()),
            (TestEnum::B, "b".to_string()),
        ])
    );
    assert_eq!(
        config.piyo,
        HashMap::from([
            (TestEnum::A, "a".to_string()),
            (TestEnum::B, "b".to_string()),
        ])
    );
    assert_eq!(
        config.qux,
        HashMap::from([(TestEnum::A, TestEnum::A), (TestEnum::B, TestEnum::B),])
    );
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_var() {
    #[derive(EnvPortal)]
    #[env_portal(
        dotenv_file = "tests/.env",
        mapping = {
            hoge: env_var::ENV_VAR_TEST_HOGE,
            hoge2: env_var::ENV_VAR_TEST_HOGE2,
            foo: Foo {
                bar: env_var::ENV_VAR_TEST_BAR,
                baz: "baz-value",
                piyo: Piyo {
                    a: env_var::ENV_VAR_TEST_PIYO_A,
                    b: env_var::ENV_VAR_TEST_PIYO_B,
                    c: env_var::ENV_VAR_TEST_PIYO_C,
                },
            },
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub hoge2: Vec<String>,
        pub foo: Foo,
    }

    pub struct Foo {
        pub bar: String,
        pub baz: String,
        pub piyo: Piyo,
    }

    pub struct Piyo {
        pub a: u32,
        pub b: bool,
        pub c: HashMap<String, String>,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-env-value".to_string());
    assert_eq!(
        config.hoge2,
        vec!["hoge2-1".to_string(), "hoge2-2".to_string()]
    );
    assert_eq!(config.foo.bar, "bar-env-value".to_string());
    assert_eq!(config.foo.baz, "baz-value".to_string());
    assert_eq!(config.foo.piyo.a, 20);
    assert_eq!(config.foo.piyo.b, true);
    assert_eq!(
        config.foo.piyo.c,
        HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ])
    );
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_match() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV1",
        dotenv_file = "tests/.env",
        mapping = {
            hoge: env_match {
                "local" => "hoge-local-value",
                "stg" | "prd" => "hoge-staging-value",
            },
            foo: env_match {
                "local" => Foo {
                    bar: env_var::ENV_MATCH_TEST_BAR,
                    baz: "baz-local-value",
                },
                "stg" | "prd" => Foo {
                    bar: env_var::ENV_MATCH_TEST_BAR,
                    baz: "baz-staging-value",
                },
            } ,
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub foo: Foo,
    }
    pub struct Foo {
        pub bar: String,
        pub baz: String,
    }

    // Values loaded from a dotenv file stay in the process env and are not overridden by later
    // loads, so remove them before switching the environment.
    unsafe {
        std::env::remove_var("ENV_MATCH_TEST_BAR");
        std::env::set_var("TEST_ENV1", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-local-value".to_string());
    assert_eq!(config.foo.bar, "bar-local-value".to_string());
    assert_eq!(config.foo.baz, "baz-local-value".to_string());

    unsafe {
        std::env::remove_var("ENV_MATCH_TEST_BAR");
        std::env::set_var("TEST_ENV1", "stg");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-staging-value".to_string());
    assert_eq!(config.foo.bar, "bar-staging-value".to_string());
    assert_eq!(config.foo.baz, "baz-staging-value".to_string());

    unsafe {
        std::env::set_var("TEST_ENV1", "invalid");
    }
    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::InvalidEnvName)));
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_partial_match() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV2",
        dotenv_file = "tests/.env",
        mapping = {
            hoge: env_partial_match {
                "local" => "hoge-local-value",
            },
        }
    )]
    pub struct EnvConfig {
        pub hoge: Option<String>,
    }

    unsafe {
        std::env::set_var("TEST_ENV2", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, Some("hoge-local-value".to_string()));

    unsafe {
        std::env::set_var("TEST_ENV2", "stg");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, None);
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_match_with_env_var() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV1",
        dotenv_file = "tests/.env",
        mapping = {
            hoge: env_match {
                "local" => "hoge-local-value",
                _       => env_var::ENV_MATCH_TEST_HOGE,
            },
            foo: env_partial_match {
                "local" => "foo-local-value",
                "stg"   => env_var::ENV_MATCH_TEST_FOO,
            },
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub foo: Option<String>,
    }

    unsafe {
        std::env::set_var("TEST_ENV1", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-local-value");
    assert_eq!(config.foo, Some("foo-local-value".to_string()));

    unsafe {
        std::env::set_var("TEST_ENV1", "stg");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-env-match-value");
    assert_eq!(config.foo, Some("foo-env-match-value".to_string()));
}

#[test]
fn test_derive_env_portal_with_custom_value_type() {
    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            hoge: "test",
        }
    )]
    pub struct EnvConfig {
        pub hoge: MyValue,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct MyValue(String);

    impl FromEnvStrValue for MyValue {
        fn from_env_str(s: EnvStrValue) -> Result<Self> {
            Ok(Self(s.to_string()))
        }
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, MyValue("test".to_string()));
}

#[test]
fn test_derive_env_portal_with_env_include() {
    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            hoge: "test-hoge",
            other: env_include<OtherEnvConfig> {}
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub other: OtherEnvConfig,
    }

    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            foo: "test-foo",
        }
    )]
    pub struct OtherEnvConfig {
        pub foo: String,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "test-hoge".to_string());
    assert_eq!(config.other.foo, "test-foo".to_string());
}

#[test]
fn test_derive_env_portal_with_env_include_and_override_fields() {
    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            hoge: "test-hoge",
            other: env_include<OtherEnvConfig> {
                foo: "override-foo",
                baz: "override-baz",
            }
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub other: OtherEnvConfig,
    }

    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            foo: "test-foo",
            bar: "test-bar",
            baz: "test-baz",
        }
    )]
    pub struct OtherEnvConfig {
        pub foo: String,
        pub bar: String,
        pub baz: String,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "test-hoge".to_string());
    assert_eq!(config.other.foo, "override-foo".to_string());
    assert_eq!(config.other.bar, "test-bar".to_string());
    assert_eq!(config.other.baz, "override-baz".to_string());
}

#[test]
fn test_derive_env_portal_with_none() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        hoge: None,
    })]
    pub struct EnvConfig {
        pub hoge: Option<String>,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, None);
}

#[test]
#[serial]
fn test_derive_env_portal_with_nested_env_match() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV3",
        mapping = {
            hoge: env_match {
                "local" | "stg" => env_match {
                    "local" => "hoge-local-value",
                    _ => "hoge-staging-value"
                },
                _ => "hoge-other-value"
            },
            other: env_match {
                "local" => env_include<OtherEnvConfig> {
                    foo: "override-foo",
                },
                _ => env_include<OtherEnvConfig>
            },
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
        pub other: OtherEnvConfig,
    }

    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        foo: "test-foo",
        bar: "test-bar",
    })]
    pub struct OtherEnvConfig {
        pub foo: String,
        pub bar: String,
    }

    unsafe {
        std::env::set_var("TEST_ENV3", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-local-value");
    assert_eq!(config.other.foo, "override-foo");
    assert_eq!(config.other.bar, "test-bar");

    unsafe {
        std::env::set_var("TEST_ENV3", "stg");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-staging-value");
    assert_eq!(config.other.foo, "test-foo");

    unsafe {
        std::env::set_var("TEST_ENV3", "prd");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-other-value");
}

#[test]
#[serial]
fn test_derive_env_portal_with_missing_dotenv_file() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV4",
        dotenv_file = "tests/not-found.env",
        mapping = {
            hoge: "hoge-value",
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
    }

    unsafe {
        std::env::set_var("TEST_ENV4", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-value");
}

#[test]
#[serial]
fn test_derive_env_portal_without_env_specific_dotenv_file() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "TEST_ENV4",
        dotenv_file = "tests/.env",
        mapping = {
            bar: env_var::ENV_MATCH_TEST_BAR,
        }
    )]
    pub struct EnvConfig {
        pub bar: String,
    }

    unsafe {
        std::env::remove_var("ENV_MATCH_TEST_BAR");
        std::env::set_var("TEST_ENV4", "prd");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.bar, "bar-env-value");
}

#[test]
#[serial]
fn test_derive_env_portal_with_broken_dotenv_file() {
    #[derive(EnvPortal)]
    #[env_portal(
        dotenv_file = "tests/broken.env",
        mapping = {
            hoge: "hoge-value",
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
    }

    let result = EnvConfig::from_env();
    assert!(matches!(result, Err(Error::DotEnvyError(_))));
}

#[test]
#[serial]
fn test_derive_env_portal_precedence_of_process_env_and_dotenv_files() {
    clear_precedence_env();
    unsafe {
        std::env::set_var("PRECEDENCE_TEST_ENV", "local");
        std::env::set_var("PRECEDENCE_ALL", "from-process");
    }

    let config = PrecedenceConfig::from_env().unwrap();
    assert_eq!(config.all, "from-process");
    assert_eq!(config.files, "from-dotenv-local");
    assert_eq!(config.base, "from-dotenv");
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_name_only_in_dotenv() {
    clear_precedence_env();

    let config = PrecedenceConfig::from_env().unwrap();
    assert_eq!(config.files, "from-dotenv-local");
}

#[test]
#[serial]
fn test_derive_env_portal_env_name_in_process_env_wins_over_dotenv() {
    clear_precedence_env();
    unsafe {
        std::env::set_var("PRECEDENCE_TEST_ENV", "stg");
    }

    let config = PrecedenceConfig::from_env().unwrap();
    assert_eq!(config.files, "from-dotenv");
}

#[derive(EnvPortal)]
#[env_portal(
    env_name_key = "PRECEDENCE_TEST_ENV",
    dotenv_file = "tests/precedence.env",
    mapping = {
        all: env_var::PRECEDENCE_ALL,
        files: env_var::PRECEDENCE_FILES,
        base: env_var::PRECEDENCE_BASE,
    }
)]
pub struct PrecedenceConfig {
    pub all: String,
    pub files: String,
    pub base: String,
}

fn clear_precedence_env() {
    for key in [
        "PRECEDENCE_TEST_ENV",
        "PRECEDENCE_ALL",
        "PRECEDENCE_FILES",
        "PRECEDENCE_BASE",
    ] {
        unsafe {
            std::env::remove_var(key);
        }
    }
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_var_for_option_field() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        name: env_var::OPTION_TEST_NAME,
        port: env_var::OPTION_TEST_PORT,
        literal: "literal-value",
    })]
    pub struct EnvConfig {
        pub name: Option<String>,
        pub port: Option<u32>,
        pub literal: Option<String>,
    }

    unsafe {
        std::env::set_var("OPTION_TEST_NAME", "name-value");
        std::env::set_var("OPTION_TEST_PORT", "8080");
    }
    let config = EnvConfig::from_env_with(None).unwrap();
    assert_eq!(config.name, Some("name-value".to_string()));
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.literal, Some("literal-value".to_string()));

    unsafe {
        std::env::set_var("OPTION_TEST_NAME", "");
        std::env::set_var("OPTION_TEST_PORT", " ");
    }
    let config = EnvConfig::from_env_with(None).unwrap();
    assert_eq!(config.name, None);
    assert_eq!(config.port, None);

    unsafe {
        std::env::remove_var("OPTION_TEST_PORT");
    }
    let result = EnvConfig::from_env_with(None);
    assert!(matches!(
        result,
        Err(Error::EnvVarError {
            var,
            field: Some("port"),
            source: std::env::VarError::NotPresent,
        }) if var == "OPTION_TEST_PORT"
    ));
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_var_opt() {
    #[derive(EnvPortal)]
    #[env_portal(mapping = {
        name: env_var_opt::OPTION_TEST_NAME,
        server: Server {
            port: env_var_opt::OPTION_TEST_PORT,
        },
    })]
    pub struct EnvConfig {
        pub name: Option<String>,
        pub server: Server,
    }

    pub struct Server {
        pub port: Option<u32>,
    }

    unsafe {
        std::env::set_var("OPTION_TEST_NAME", "name-value");
        std::env::set_var("OPTION_TEST_PORT", "8080");
    }
    let config = EnvConfig::from_env_with(None).unwrap();
    assert_eq!(config.name, Some("name-value".to_string()));
    assert_eq!(config.server.port, Some(8080));

    unsafe {
        std::env::set_var("OPTION_TEST_NAME", "");
        std::env::remove_var("OPTION_TEST_PORT");
    }
    let config = EnvConfig::from_env_with(None).unwrap();
    assert_eq!(config.name, None);
    assert_eq!(config.server.port, None);

    unsafe {
        std::env::set_var("OPTION_TEST_PORT", "not-a-number");
    }
    let err = EnvConfig::from_env_with(None).err().unwrap();
    assert!(matches!(err, Error::ValConversionError(_)));
    assert!(err.to_string().contains("server.port"), "{err}");
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_match_for_option_field() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "OPTION_TEST_ENV",
        mapping = {
            key: env_match {
                "local" | "docker" => env_var_opt::OPTION_TEST_KEY,
                "stg" | "prod" => env_var::OPTION_TEST_KEY,
            },
            nested: env_match {
                "local" => None,
                _ => env_match {
                    "stg" => env_var_opt::OPTION_TEST_KEY,
                    _ => "nested-value",
                },
            },
        }
    )]
    pub struct EnvConfig {
        pub key: Option<String>,
        pub nested: Option<String>,
    }

    let env = |name: &str| Some(name.to_string());

    unsafe {
        std::env::remove_var("OPTION_TEST_KEY");
    }
    let config = EnvConfig::from_env_with(env("local")).unwrap();
    assert_eq!(config.key, None);
    assert_eq!(config.nested, None);
    let config = EnvConfig::from_env_with(env("docker")).unwrap();
    assert_eq!(config.key, None);
    assert_eq!(config.nested, Some("nested-value".to_string()));
    let result = EnvConfig::from_env_with(env("prod"));
    assert!(matches!(
        result,
        Err(Error::EnvVarError {
            field: Some("key"),
            ..
        })
    ));

    unsafe {
        std::env::set_var("OPTION_TEST_KEY", "");
    }
    let config = EnvConfig::from_env_with(env("local")).unwrap();
    assert_eq!(config.key, None);
    let config = EnvConfig::from_env_with(env("stg")).unwrap();
    assert_eq!(config.key, None);
    assert_eq!(config.nested, None);

    unsafe {
        std::env::set_var("OPTION_TEST_KEY", "key-value");
    }
    for name in ["local", "docker", "stg", "prod"] {
        let config = EnvConfig::from_env_with(env(name)).unwrap();
        assert_eq!(config.key, Some("key-value".to_string()), "{name}");
    }
    let config = EnvConfig::from_env_with(env("stg")).unwrap();
    assert_eq!(config.nested, Some("key-value".to_string()));
}

#[test]
fn test_derive_env_portal_with_env_partial_match_for_numeric_option_field() {
    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "OPTION_TEST_ENV",
        mapping = {
            port: env_partial_match {
                "local" => "8080",
            },
        }
    )]
    pub struct EnvConfig {
        pub port: Option<u32>,
    }

    let config = EnvConfig::from_env_with(Some("local".to_string())).unwrap();
    assert_eq!(config.port, Some(8080));
    let config = EnvConfig::from_env_with(Some("stg".to_string())).unwrap();
    assert_eq!(config.port, None);
}

#[test]
#[serial]
fn test_derive_env_portal_with_env_match_guard() {
    fn enabled() -> bool {
        std::env::var("GUARD_TEST_ENABLED").is_ok()
    }

    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "GUARD_TEST_ENV",
        mapping = {
            mode: env_match {
                "local" | "docker" if enabled() => "enabled",
                "local" | "docker" => "disabled",
                _ => "default",
            },
            banner: env_partial_match {
                "local" if enabled() => "banner",
            },
        }
    )]
    pub struct EnvConfig {
        pub mode: String,
        pub banner: Option<String>,
    }

    unsafe {
        std::env::set_var("GUARD_TEST_ENABLED", "1");
    }
    let config = EnvConfig::from_env_with(Some("docker".to_string())).unwrap();
    assert_eq!(config.mode, "enabled");
    assert_eq!(config.banner, None);
    let config = EnvConfig::from_env_with(Some("local".to_string())).unwrap();
    assert_eq!(config.banner, Some("banner".to_string()));

    unsafe {
        std::env::remove_var("GUARD_TEST_ENABLED");
    }
    let config = EnvConfig::from_env_with(Some("local".to_string())).unwrap();
    assert_eq!(config.mode, "disabled");
    assert_eq!(config.banner, None);
    let config = EnvConfig::from_env_with(Some("prd".to_string())).unwrap();
    assert_eq!(config.mode, "default");
}
