use env_logger::Env;
// use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
