use std::sync::mpsc::{channel, Receiver};
use std::{path::PathBuf, sync::mpsc::Sender};

use anyhow::Result;
use notify::{Event, EventHandler, Watcher};

use crate::{WaitOptions, Waitable};

pub struct FileWaiter {
    pub path: PathBuf,
}

impl FileWaiter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Waitable for FileWaiter {
    async fn wait(&self, _: &WaitOptions) -> Result<()> {
        let (file_exists_handler, rx) = FileExistsHandler::new();
        let mut watcher = notify::recommended_watcher(file_exists_handler).unwrap();
        let parent = self.path.parent().unwrap();

        watcher
            .watch(parent, notify::RecursiveMode::NonRecursive)
            .unwrap();

        if rx.recv().is_ok() {
            watcher.unwatch(parent).unwrap();
        }

        Ok(())
    }
}

struct FileExistsHandler {
    tx: Sender<()>,
}

impl FileExistsHandler {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel();

        (Self { tx }, rx)
    }
}

impl EventHandler for FileExistsHandler {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        if let Ok(event) = event {
            if let notify::EventKind::Create(_) = event.kind {
                self.tx.send(()).unwrap();
            }
        }
    }
}
