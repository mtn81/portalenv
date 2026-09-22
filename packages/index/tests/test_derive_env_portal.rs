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

    unsafe {
        std::env::set_var("TEST_ENV1", "local");
    }
    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "hoge-local-value".to_string());
    assert_eq!(config.foo.bar, "bar-local-value".to_string());
    assert_eq!(config.foo.baz, "baz-local-value".to_string());

    unsafe {
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
