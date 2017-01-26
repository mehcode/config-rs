extern crate config;

fn main() {
    // Read configuration from $(cwd)/Cargo.toml
    config::merge(config::File::with_name("Cargo")).unwrap();

    println!("package.name = {:?}", config::get_str("package.name"));
    println!("package.version = {:?}", config::get_str("package.version"));
}
