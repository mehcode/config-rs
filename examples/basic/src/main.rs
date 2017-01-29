extern crate config;

fn main() {
    // Set defaults for `window.width` and `window.height`
    config::set_default("window.title", "Basic").unwrap();
    config::set_default("window.width", 640).unwrap();
    config::set_default("window.height", 480).unwrap();
    config::set_default("debug", true).unwrap();

    // Note that you can retrieve the stored values as any type as long
    // as there exists a reasonable conversion
    println!("window.title  : {:?}", config::get_str("window.title"));
    println!("window.width  : {:?}", config::get_str("window.width"));
    println!("window.width  : {:?}", config::get_int("window.width"));
    println!("debug         : {:?}", config::get_bool("debug"));
    println!("debug         : {:?}", config::get_str("debug"));
    println!("debug         : {:?}", config::get_int("debug"));

    // Attempting to get a value as a type that cannot be reasonably
    // converted to will return None
    println!("window.title  : {:?}", config::get_bool("window.title"));

    // Instead of using a get_* function you can get the variant
    // directly
    println!("debug         : {:?}", config::get("debug"));
    println!("debug         : {:?}",
             config::get("debug").unwrap().as_int());

    // Attempting to get a value that does not exist will return None
    println!("not-found     : {:?}", config::get("not-found"));
}
