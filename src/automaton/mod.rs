pub mod automaton_manager;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

pub use automaton_manager::*;

pub mod action;
pub use action::*;

pub mod state;
use derive_more::From;
pub use state::*;

pub mod reducer;
pub use reducer::*;

pub mod effect;
pub use effect::*;

pub mod action_logger;

#[derive(From, Clone)]
pub struct Logger(slog::Logger);

impl Default for Logger {
    fn default() -> Self {
        Self(slog::Logger::root(slog::Discard, slog::o!()))
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Logger")
    }
}

impl Deref for Logger {
    type Target = slog::Logger;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Logger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
