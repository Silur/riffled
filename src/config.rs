pub struct Config {
    pub host: String,
    pub port: u16, 
    pub server_mode: bool
}

impl Config {
    pub fn new() -> Config {
        Config { 
            host: String::from("0.0.0.0"),
            port: 1979,
            server_mode: true, 
        }
    }
}
