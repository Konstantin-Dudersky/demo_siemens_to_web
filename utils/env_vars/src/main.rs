use env_vars::{create_env_file, Config};

fn main() {
    create_env_file::<Config>(".env.example").unwrap();
}
