pub struct Config {
    pub host: String,
    pub port: u16, 
    pub server_mode: bool,
    pub primary_server: String
}

impl Config {
    pub fn new() -> Config {
        Config { 
            host: String::from("0.0.0.0"),
            port: 1979,
            server_mode: true,
            primary_server: String::from("riffle.sh")
        }
    }
}
