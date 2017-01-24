
// NOTE: This is just for my testing / play right now. Examples will be made at examples/ soon.

extern crate config;

fn main() {
    let mut c = config::Config::new();

    c.merge(config::File::with_name("Settings")).unwrap();

    println!("debug = {:?}", c.get_str("process.debug"));
    println!("debug = {:?}", c.get_bool("process.debug"));
}
