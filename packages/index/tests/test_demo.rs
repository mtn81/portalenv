#[test]
#[allow(unused)]
fn test() {
    use portalenv::*;
    use std::collections::{HashMap, HashSet};

    #[derive(EnvPortal)]
    #[env_portal(
        env_name_key = "APP_ENV",
        mapping = {
            app_name: "Demo",
            server: ServerConfig {
                host: env_var::SERVER_HOST,
                port: env_var::SERVER_PORT,
            },
            email: EmailConfig {
                address: env_match {
                    "local" | "stg" => "portalenv-test@xxx.yyy",
                    "prd" => "portalenv@xxx.yyy",
                },
                title: env_partial_match {
                    "prd" => "portalenv mail"
                }
            },
            demo: DemoConfig {
                vec: ["this", "is", "a", "demo"],
                set: [1, 2, 3],
                map: {
                    count_a: 100,
                    count_b: 200,
                },
                child: DemoChild {
                    name: "portal env",
                    amount: 20,
                    size: SizeType::Small,
                },
            }
        }
    )]
    pub struct EnvConfig {
        app_name: String,
        server: ServerConfig,
        email: EmailConfig,
        demo: DemoConfig,
    }
    pub struct ServerConfig {
        host: String,
        port: u32,
    }
    pub struct EmailConfig {
        address: EmailAddress,
        title: Option<String>,
    }
    pub struct EmailAddress(String);
    impl FromEnvStrValue for EmailAddress {
        fn from_env_str(s: EnvStrValue) -> Result<Self> {
            // return Err(...) here if the value is invalid
            Ok(EmailAddress(s.into()))
        }
    }

    pub struct DemoConfig {
        vec: Vec<String>,
        set: HashSet<u32>,
        map: HashMap<String, u32>,
        child: DemoChild,
    }

    pub struct DemoChild {
        name: String,
        amount: u32,
        size: SizeType,
    }

    #[derive(EnvEnum)]
    pub enum SizeType {
        Small,
        Medium,
        Large,
    }

    fn main() {
        let _config = EnvConfig::from_env().unwrap();
    }
}
