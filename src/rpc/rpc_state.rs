#[derive(Clone, Debug)]
pub enum RpcState {
    Idle,
    Pending,
    Success,
}

impl Default for RpcState {
    fn default() -> Self {
        RpcState::Idle
    }
}
