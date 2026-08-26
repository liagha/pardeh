use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tokio::sync::broadcast;

use crate::html::{Node, div};

#[derive(Clone, Serialize)]
pub struct Patch {
    pub region: String,
    pub html: String,
}

type Draw = Arc<dyn Fn(&Signals) -> Node + Send + Sync>;

struct Inner {
    values: Mutex<HashMap<String, Box<dyn Any + Send + Sync>>>,
    draws: Mutex<HashMap<String, Draw>>,
    watchers: Mutex<HashMap<String, HashSet<String>>>,
    feed: broadcast::Sender<Patch>,
}

#[derive(Clone)]
pub struct Signals(Arc<Inner>);

thread_local! {
    static TRACKING: RefCell<Option<(String, Vec<String>)>> = const { RefCell::new(None) };
}

impl Default for Signals {
    fn default() -> Self {
        Self::new()
    }
}

impl Signals {
    #[must_use]
    pub fn new() -> Self {
        let (feed, _) = broadcast::channel(256);
        Self(Arc::new(Inner {
            values: Mutex::new(HashMap::new()),
            draws: Mutex::new(HashMap::new()),
            watchers: Mutex::new(HashMap::new()),
            feed,
        }))
    }

    pub fn define<T: Any + Send + Sync>(&self, key: &str, value: T) {
        self.0
            .values
            .lock()
            .expect("values lock")
            .insert(key.to_string(), Box::new(value));
    }

    pub fn set<T: Any + Send + Sync>(&self, key: &str, value: T) {
        self.0
            .values
            .lock()
            .expect("values lock")
            .insert(key.to_string(), Box::new(value));
        for id in self.interested(key) {
            self.repaint(&id);
        }
    }

    pub fn get<T: Clone + Any + Send + Sync>(&self, key: &str) -> T {
        TRACKING.with(|cell| {
            if let Some((_, seen)) = cell.borrow_mut().as_mut() {
                seen.push(key.to_string());
            }
        });
        let values = self.0.values.lock().expect("values lock");
        values
            .get(key)
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
            .unwrap_or_else(|| panic!("undefined signal {key}"))
    }

    pub fn region(
        &self,
        id: &str,
        draw: impl Fn(&Signals) -> Node + Send + Sync + 'static,
    ) -> Node {
        let draw: Draw = Arc::new(draw);
        self.0
            .draws
            .lock()
            .expect("draws lock")
            .insert(id.to_string(), Arc::clone(&draw));
        TRACKING.with(|cell| *cell.borrow_mut() = Some((id.to_string(), Vec::new())));
        let body = draw(self);
        let seen = TRACKING.with(|cell| cell.borrow_mut().take());
        if let Some((_, keys)) = seen {
            let mut watchers = self.0.watchers.lock().expect("watchers lock");
            for key in keys {
                watchers.entry(key).or_default().insert(id.to_string());
            }
        }
        div().attr("data-pardeh", id).kid(body)
    }

    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Patch> {
        self.0.feed.subscribe()
    }

    fn repaint(&self, id: &str) {
        let draw = {
            let draws = self.0.draws.lock().expect("draws lock");
            draws.get(id).cloned()
        };
        if let Some(draw) = draw {
            let patch = Patch {
                region: id.to_string(),
                html: draw(self).render(),
            };
            let _ = self.0.feed.send(patch);
        }
    }

    fn interested(&self, key: &str) -> Vec<String> {
        self.0
            .watchers
            .lock()
            .expect("watchers lock")
            .get(key)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }
}
