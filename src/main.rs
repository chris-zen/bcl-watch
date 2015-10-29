extern crate notify;

use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::env;

fn main() {
  // Create a channel to receive the events.
  let (tx, rx) = channel();

  // Automatically select the best implementation for your platform.
  // You can also access each implementation directly e.g. INotifyWatcher.
  let mut watcher: RecommendedWatcher = Watcher::new(tx).unwrap();

  let current_path = env::current_dir().unwrap();

  watcher.watch(current_path.as_path()).unwrap();

  loop {
    match rx.recv() {
      Ok(event) => {
        println!("Recv {:?}", event);
        if let Some(path) = event.path {
            if let Ok(op) = event.op {
                println!("path: {:?}, op: {:?}", path, op);
            }
        }
      },
      Err(error) =>
        println!("Error: {}", error)
    }
  }
}
