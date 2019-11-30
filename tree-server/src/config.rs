use serde::{Serialize, Deserialize};

// server config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    address: String,
}

impl Default for Config {
    fn default() -> Self {
        let server = Server { address: String::from("0.0.0.0:9091") };
        Config { server }
    }
}