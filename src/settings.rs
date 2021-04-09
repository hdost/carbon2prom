extern crate config;
extern crate notify;

use config::*;
use log::{error, info, warn};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::time::Duration;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

// TODO: Should this instead be triggered by a signal
pub async fn watch_config(config_file: String) {
    info!("Loading config: {}", config_file);
    // Create a channel to monitor file changes.
    let (tx, rx) = channel();

    // Perform initial file load.
    SETTINGS
        .write()
        .unwrap()
        .merge(File::with_name(&config_file))
        .unwrap();

    // App should die if we can't watch the file.
    let mut watcher: RecommendedWatcher = match Watcher::new(tx, Duration::from_secs(2)) {
        Ok(w) => w,
        Err(e) => {
            error!("{:}", e);
            panic!();
        }
    };

    // Add a path to be watched.
    // TODO: Validate that it's not a dir.
    watcher
        .watch(config_file, RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path)) => {
                println!(" * {:?} written; refreshing configuration ...", path);
                SETTINGS.write().unwrap().refresh().unwrap();
            }
            Ok(some) => {
                warn!("Ignored: {:?}", some);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
