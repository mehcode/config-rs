#![allow(deprecated)]
use config::{Config, File};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("examples/watch/Settings.toml")).unwrap();

        settings
    });
}

fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}

fn watch() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("examples/watch/Settings.toml", RecursiveMode::NonRecursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                SETTINGS.write().unwrap().refresh().unwrap();
                show();
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}

fn main() {
    // This is just an example of what could be done, today
    // We do want this to be built-in to config-rs at some point
    // Feel free to take a crack at a PR

    show();
    watch();
}
