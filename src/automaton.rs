pub use crate::services::{Service, ServiceDefault};

pub type Store<Service> = redux_rs::Store<State, Service, Action>;