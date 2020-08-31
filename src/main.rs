extern crate sodiumoxide;
#[macro_use] extern crate log;
extern crate num;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

mod config;
mod net;
mod shuffle;
mod pir;
use net::{Client, Server, Agent};

fn main() {
    info!("reading log");
    let conf_file = match File::open("./riffle.conf") {
        Ok(a) => a,
        Err(e) => panic!("error opening config file: {}", e)
    };
    let mut reader = BufReader::new(conf_file);
    let mut line = String::new();
    let mut main_config = config::Config::new();
    if sodiumoxide::init().is_err() {
        error!("failed to initialize crypto libraries in thread-safe mode");
    }
    loop {
        if reader.read_line(&mut line).unwrap() == 0 { break; }
        let split: Vec<&str> = line.split("=").collect();
        let token = split[0].trim();
        let value = split[1].trim();

        match token {
            "mode" => { 
                main_config.server_mode = value == "server";
            },
            "host" => {
                main_config.host = String::from(value);
            },
            "port" => {
                main_config.port = value.parse().unwrap();
            },
            "primary-server" => {
                main_config.primary_server = String::from(value);
            },
            _ => error!("unexpected config param")
        }
    }

    if main_config.server_mode {
        start_server(main_config);
    } else {
        start_client(main_config);
    }
}

fn start_server(conf: config::Config) {
    let me = Server::new(conf.host, conf.port);
}

fn start_client(conf: config::Config) {
    let me = Client::new(conf.host, conf.port);
}
