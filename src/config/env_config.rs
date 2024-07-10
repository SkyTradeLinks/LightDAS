use std::env;

pub struct EnvConfig {
    rpc_url: String,
    websocket_url: String,
    database_url: String,
}

impl EnvConfig {
    pub fn get_websocket_url(&self) -> &String {
        &self.websocket_url
    }

    pub fn get_rpc_url(&self) -> &String {
        &self.rpc_url
    }

    pub fn get_database_url(&self) -> &String {
        &self.database_url
    }
}

pub fn setup_env_config() -> EnvConfig {
    let env_ws_url = "ws://127.0.0.1:8900".to_string();
    let env_rpc_url = "http://127.0.0.1:8899".to_string();
    let env_db_url = "postgres://solana:solana@localhost:5433/solana".to_string();

    EnvConfig {
        websocket_url: env_ws_url,
        rpc_url: env_rpc_url,
        database_url: env_db_url,
    }
}
