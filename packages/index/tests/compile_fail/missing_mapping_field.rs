use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(mapping = {
    foo: "foo-value",
})]
pub struct EnvConfig {
    pub foo: String,
    pub bar: String,
    pub baz: u32,
}

fn main() {}
