use env_vars::load;
use env_vars::{create_env_file, Config};

fn main() {
    create_env_file::<Config>(".env.example").unwrap();
    let config = load().unwrap();
    println!("Loaded settings: {:#?}", config);
}
