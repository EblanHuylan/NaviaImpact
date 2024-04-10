use std::sync::atomic::{AtomicU64, Ordering};

const INITIAL_GUID: u64 = 1000000000000000000;

pub struct GUIDManager {
    guid_counter: AtomicU64,
}

impl GUIDManager {
    pub fn new() -> Self {
        Self {
            guid_counter: AtomicU64::new(INITIAL_GUID),
        }
    }

    pub fn next(&self) -> u64 {
        self.guid_counter.fetch_add(1, Ordering::SeqCst) + 1
    }
}
