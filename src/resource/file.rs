use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::mpsc::{channel, Receiver};

use anyhow::Result;
use notify::{Event, EventHandler, Watcher};

use crate::{WaitOptions, Waitable};

#[derive(Clone)]
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
        let mut watcher = notify::recommended_watcher(file_exists_handler)?;

        if let Some(parent) = self.path.parent() {
            watcher.watch(parent, notify::RecursiveMode::NonRecursive)?;

            if rx.recv().is_ok() {
                watcher.unwatch(parent)?;
            }
        } else {
            watcher.watch(&self.path, notify::RecursiveMode::NonRecursive)?;

            if rx.recv().is_ok() {
                watcher.unwatch(&self.path)?;
            }
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
                self.tx.send(()).expect("Channel dropped.");
            }
        }
    }
}
