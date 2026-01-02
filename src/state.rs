use crate::puzzle::PuzzleConfig;
use once_cell::sync::Lazy;
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Clone)]
pub struct State {
    pub current_puzzle_index: u32,
    pub puzzle_config: PuzzleConfig,
}

pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::capture()
            );
            // pause so you can attach a debugger (gdb/rust-gdb) to inspect threads/stacks
            std::thread::sleep(std::time::Duration::from_secs(2));
            // fallback to blocking lock so program can continue after inspection
            APP_STATE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => {
            // preserve original behavior on poisoned lock
            APP_STATE.lock().unwrap()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let puzzle_config = PuzzleConfig::default();
        State {
            current_puzzle_index: 0,
            puzzle_config,
        }
    }
}
