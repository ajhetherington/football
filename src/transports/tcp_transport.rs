use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream}, os::unix::net::SocketAddr,
    str::from_utf8, thread, time::Duration
};


use macroquad::miniquad::log;

use super::base::Transport;

pub struct TCPTransport {
    sender: String,
    receiver: String,
    frame_iter: usize,
}

impl TCPTransport {
    pub fn new(client_addr: &str, server_addr: &str) -> Self {
        TCPTransport {
            sender: client_addr.to_owned(),
            receiver: server_addr.to_owned(),
            frame_iter: 0,
        }
    }
}

impl Transport for TCPTransport {
    async fn get_action(
        &self,
        state: &crate::gamestate::GameState,
    ) -> Vec<crate::agent::AgentAction> {
        let output = serde_json::to_string(state).unwrap_or("default".to_owned());
        let output_bytes = output.as_bytes();
        println!("here, just about to send stuff to addr {}", self.sender);
        let sender = TcpListener::bind(self.sender.clone()).unwrap();
        for stream in sender.incoming() {
            // write
            //  stream in sender.incoming() {
            match stream {
                Ok(mut stream) => {
                    let size = (output_bytes.len() as u32).to_be_bytes();
                    println!("{:?}", size.clone());
                    stream.write(&size).unwrap();
                    let _n_bytes = stream.write_all(output_bytes).unwrap();
                    println!("finished sending");
                    break;
                }
                Err(_) => {
                    panic!("Failed to get tcp stream for sending game state")
                }
            }
        }

        // read
        let receiver = TcpListener::bind(self.receiver.clone()).unwrap();
        let mut buffer: Vec<u8> = vec![];
        println!("just about to get things {}", self.receiver);
        for stream in receiver.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("found a tcp stream");
                    let mut tmp_buffer = [0; 4];
                    stream.read_exact(&mut tmp_buffer).unwrap();
                    println!("just read the length");

                    let length = u32::from_be_bytes(tmp_buffer);
                    println!("found {:?} bits to read", length);
                    // cannot use vec::with_capacity as that instantiates as 0 length
                    buffer = vec![0; length as usize];
                    stream.read_exact(&mut buffer).unwrap();
                    println!("{:?}", buffer);
                    break;
                }
                Err(_) => {
                    panic!("Failed to get tcp stream for getting actions")
                }
            }
        }

        let value = from_utf8(&buffer).unwrap();
        println!("value: {}", value);
        thread::sleep(Duration::from_millis(100));
        serde_json::from_str(value).unwrap()

    }
}
