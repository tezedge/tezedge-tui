use clap::Parser;
use std::str::FromStr;
use url::Url;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct TuiArgs {
    /// Url of the node's RPC server
    #[clap(parse(try_from_str), default_value_t=Url::from_str("http://127.0.0.1:18732").unwrap())]
    pub node: Url,

    /// URL of the node's websocket server
    #[clap(parse(try_from_str), default_value_t=Url::from_str("ws://127.0.0.1:4927").unwrap())]
    pub websocket: Url,
}
