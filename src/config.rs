use std::env;

pub fn get_rpc_url() -> String {
    env::var("RPC_URL").expect("RPC_URL must be set")
}

pub fn get_private_key() -> String {
    env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set")
}
