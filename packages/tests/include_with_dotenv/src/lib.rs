use portalenv::*;

use default_dotenv_test::EnvConfig as AnotherEnvConfig;

#[test]
fn test_derive_env_portal_with_env_include_and_dotenv_file() {
    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            foo: env_var::DEFAULT_FOO,
            another: env_include<AnotherEnvConfig>,
        }
    )]
    pub struct EnvConfig {
        pub foo: String,
        pub another: AnotherEnvConfig,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.foo, "default-foo-value".to_string());
    assert_eq!(config.another.hoge, "default-hoge-value".to_string());
}
