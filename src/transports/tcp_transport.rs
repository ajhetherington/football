use std::{env, vec};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

use crate::agent::AgentAction;
use crate::gamestate::GameState;
use crate::transports::base::Transport;

#[derive(Debug)]
pub struct TCPTransport {
    host: String,
    port: u16,
    frame_iter: usize,
    socket: SocketAddr,
    listener: TcpListener,
    eof: String,
}

impl TCPTransport {
    pub fn new(host: &str, port: u16) -> Self {
        let ip: (u8, u8, u8, u8) = match host {
            "localhost" => (127, 0, 0, 1),
            _ => {
                let splits: Vec<&str> = host.split('.').collect();
                if splits.len() != 4 {
                    panic!("{host} is not a valid ip");
                }
                (
                    splits[0].parse().unwrap(),
                    splits[1].parse().unwrap(),
                    splits[2].parse().unwrap(),
                    splits[3].parse().unwrap(),
                )
            }
        };
        let _ip = IpAddr::from(Ipv4Addr::new(ip.0, ip.1, ip.2, ip.3));

        let socket = SocketAddr::new(_ip, port);
        let mut _listener = TcpListener::bind(socket);

        loop {
            match _listener {
                Err(e) => std::thread::sleep(Duration::from_millis(10)),
                _ => break,
            }
            _listener = TcpListener::bind(socket);
        }
        let listener = _listener.unwrap();

        let eof = match env::var("END_OF_FILE") {
            Ok(val) => val,
            Err(_) => "EOF".to_owned(),
        };

        TCPTransport {
            host: host.to_string(),
            port,
            frame_iter: 0,
            listener,
            socket,
            eof,
        }
    }
}

impl Transport for TCPTransport {
    async fn get_action(&self, state: &GameState) -> Vec<AgentAction> {
        let mut output = serde_json::to_string(state).unwrap_or("default".to_owned());
        output += &self.eof;
        let out_bytes = output.as_bytes();

        // write
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _n_bytes = stream.write_all(out_bytes).unwrap();
                    break;
                }
                Err(e) => {
                    panic!("failed to get tcp stream")
                }
            }
        }
        println!("before read");
        let mut buffer = String::new();
        // read
        for stream in self.listener.incoming() {
            println!("polling stream");
            match stream {
                Ok(stream) => {
                    println!("found stream");
                    let mut reader = BufReader::new(&stream);
                    let n_bytes = reader.read_line(&mut buffer).unwrap();
                    println!("n bytes: {:?}", n_bytes);
                    break;
                }
                Err(e) => continue,
            }
        }
        // let output = String::from_utf8(buffer.clone()).unwrap();
        println!("received buffer: {:?}", buffer.clone());
        // println!("output: {:?}", output);
        return serde_json::from_str(&output).unwrap();
    }
}
