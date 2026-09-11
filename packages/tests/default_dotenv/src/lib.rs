use portalenv::*;

#[test]
fn test_derive_env_portal_with_default_dotenv_file() {
    #[derive(EnvPortal)]
    #[env_portal(
        mapping = {
            hoge: env_var::DEFAULT_HOGE,
        }
    )]
    pub struct EnvConfig {
        pub hoge: String,
    }

    let config = EnvConfig::from_env().unwrap();
    assert_eq!(config.hoge, "default-hoge-value".to_string());
}
