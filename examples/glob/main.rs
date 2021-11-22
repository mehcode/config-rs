use config::*;
use glob::glob;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    // Option 1
    // --------
    // Gather all conf files from conf/ manually
    let mut settings = Config::default();
    settings
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .merge(File::with_name("examples/glob/conf/00-default.toml"))
        .unwrap()
        .merge(File::from(Path::new("examples/glob/conf/05-some.yml")))
        .unwrap()
        .merge(File::from(Path::new("examples/glob/conf/99-extra.json")))
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // Option 2
    // --------
    // Gather all conf files from conf/ manually, but put in 1 merge call.
    let mut settings = Config::default();
    settings
        .merge(vec![
            File::with_name("examples/glob/conf/00-default.toml"),
            File::from(Path::new("examples/glob/conf/05-some.yml")),
            File::from(Path::new("examples/glob/conf/99-extra.json")),
        ])
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // Option 3
    // --------
    // Gather all conf files from conf/ using glob and put in 1 merge call.
    let mut settings = Config::default();
    settings
        .merge(
            glob("examples/glob/conf/*")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}
