use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    foo "foo-value",
})]
pub struct EnvConfig {
    pub foo: String,
}

fn main() {}
