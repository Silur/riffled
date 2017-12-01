extern crate sodiumoxide;
use std::net::{TcpListener, TcpStream};
use std::io::Error as IOError;
use sodiumoxide::crypto::box_::{PublicKey, SecretKey};
use sodiumoxide::crypto::box_ as PkBox;
use sodiumoxide::crypto::secretbox::Key;
use sodiumoxide::crypto::secretbox;


pub struct Server {
    pub sock_addr: String,
    pub keypairs: Vec<(PublicKey, SecretKey)>,
    pub permutation: Vec<usize>,
    pub known_clients: Vec<Client>
}

pub struct Client {
    pub sock_addr: String,
    pub known_servers: Vec<Server>,
    pub aead_keys: Vec<Key>
}

pub trait Agent {
    fn bind(&self) -> Result<TcpListener, IOError>;
    fn broadcast(&self, data: &[u8]);
    fn gen_keys(&mut self);
}

impl Agent for Server {
    
    fn bind(&self) -> Result<TcpListener, IOError> {
        TcpListener::bind(&self.sock_addr)
    }

    fn gen_keys(&mut self) {
        for _ in 0..self.known_clients.len() {
            self.keypairs.push(PkBox::gen_keypair());
        }
    }
    fn broadcast(&self, data: &[u8]) {
        for client in &self.known_clients {
            use std::io::Write;
            let mut stream = match TcpStream::connect(&client.sock_addr) {
                Ok(stream) => stream,
                Err(e) => panic!("error connecting to client: {}", e)
            };
            if !stream.write(data).is_ok() {
                error!("falied to write into socket");
            }
        }
    }
}
impl Agent for Client {

    fn bind(&self) -> Result<TcpListener, IOError> {
        TcpListener::bind(&self.sock_addr)
    }

    fn gen_keys(&mut self) {
        for _ in 0..self.known_servers.len() {
            self.aead_keys.push(secretbox::gen_key());
        }
    }
    fn broadcast(&self, data: &[u8]) {
        unimplemented!();
    }
}
impl Server {
    pub fn new(host: String, port: u16) -> Server {
        use std::fmt::Write;
        let mut normalized_addr = String::new();
        write!(&mut normalized_addr, "{}:{}", host, port)
            .expect("failed to parse server sock addr");
        Server {
            sock_addr: normalized_addr,
            keypairs: Vec::new(),
            permutation: Vec::new(),
            known_clients: Vec::new()
        }
    }
}
impl Client {
    pub fn new(host: String, port: u16) -> Client {
        use std::fmt::Write;
        let mut normalized_addr = String::new();
        write!(&mut normalized_addr, "{}:{}", host, port)
            .expect("failed to parse client sock addr");
        Client {
            sock_addr: normalized_addr,
            aead_keys: Vec::new(),
            known_servers: Vec::new()
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn server_genkey_1024() {
        let mut server = Server::new(String::from("127.0.0.1"), 1979);
        for _ in 0..1024 {
            server.known_clients.push(Client::new(String::from("test"), 1979));
        }
        println!("server len: {}", server.known_clients.len());
        server.gen_keys();
        assert_eq!(server.keypairs.len(), 1024);
    }
    #[test]
    fn client_genkey_1024() {
        let mut client = Client::new(String::from("127.0.0.1"), 1979);
        for _ in 0..1024 {
            client.known_servers.push(Server::new(String::from("test"), 1979));
        }
        println!("client len: {}", client.known_servers.len());
        client.gen_keys();
        assert_eq!(client.aead_keys.len(), 1024);
    }
}
