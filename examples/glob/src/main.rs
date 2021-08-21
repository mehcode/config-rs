use std::path::Path;
use std::collections::HashMap;
use config::*;
use glob::glob;

fn main() {
    // Option 1
    // --------
    // Gather all conf files from conf/ manually
    let mut settings = Config::default();
    settings
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .merge(File::with_name("conf/00-default.toml")).unwrap()
        .merge(File::from(Path::new("conf/05-some.yml"))).unwrap()
        .merge(File::from(Path::new("conf/99-extra.json"))).unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.try_into::<HashMap<String, String>>().unwrap());

    // Option 2
    // --------
    // Gather all conf files from conf/ manually, but put in 1 merge call.
    let mut settings = Config::default();
    settings
        .merge(vec![File::with_name("conf/00-default.toml"),
                    File::from(Path::new("conf/05-some.yml")),
                    File::from(Path::new("conf/99-extra.json"))])
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.try_into::<HashMap<String, String>>().unwrap());

    // Option 3
    // --------
    // Gather all conf files from conf/ using glob and put in 1 merge call.
    let mut settings = Config::default();
    settings
        .merge(glob("conf/*")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())
        .unwrap();

    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------",
             settings.try_into::<HashMap<String, String>>().unwrap());
}
