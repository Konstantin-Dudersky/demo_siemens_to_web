use env_vars::load;

fn main() {
    let config = load();
    println!("{:#?}", config);
}
