extern crate config;

use config::*;

fn main() {
    let mut c = Config::default();

    // // Set defaults for `window.width` and `window.height`
    // c.set_default("window.title", "Basic").unwrap();
    // c.set_default("window.width", 640).unwrap();
    // c.set_default("window.height", 480).unwrap();
    // c.set_default("debug", true).unwrap();

    // // Note that you can retrieve the stored values as any type as long
    // // as there exists a reasonable conversion
    // println!("window.title  : {:?}", c.get_str("window.title"));
    // println!("window.width  : {:?}", c.get_str("window.width"));
    // println!("window.width  : {:?}", c.get_int("window.width"));
    // println!("debug         : {:?}", c.get_bool("debug"));
    // println!("debug         : {:?}", c.get_str("debug"));
    // println!("debug         : {:?}", c.get_int("debug"));

    // // Attempting to get a value as a type that cannot be reasonably
    // // converted to will return None
    // println!("window.title  : {:?}", c.get_bool("window.title"));

    // // Instead of using a get_* function you can get the variant
    // // directly
    // println!("debug         : {:?}", c.get("debug"));
    // println!("debug         : {:?}",
    //          c.get("debug").unwrap().into_int());

    // // Attempting to get a value that does not exist will return None
    // println!("not-found     : {:?}", c.get("not-found"));
}
