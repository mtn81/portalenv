use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    port: env_var_opt::PORT,
})]
pub struct EnvConfig {
    pub port: u32,
}

fn main() {}
