use portalenv::*;

#[derive(EnvPortal)]
#[env_portal(dotenv_fle = ".env")]
pub struct EnvConfig {}

fn main() {}
