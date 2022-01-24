use enum_kinds::EnumKind;
use derive_more::From;
pub use redux_rs::{ActionId, EnablingCondition};

pub type ActionWithMeta = redux_rs::ActionWithMeta<Action>;

#[derive(Debug, Clone)]
pub struct InitAction {}

impl EnablingCondition<State> for InitAction {
    fn is_enabled(&self, _: &State) -> bool {
        false
    }
}

#[derive(
    EnumKind,
    strum_macros::AsRefStr,
    strum_macros::IntoStaticStr,
    From,
    Debug,
    Clone,
)]
#[enum_kind(
    ActionKind,
    derive(
        strum_macros::EnumIter,
        strum_macros::Display,
        Hash
    )
)]
pub enum Action {
    Init(InitAction)
}