use num_bigint::Sign;
use protohackers::tcp_accept_and_spawn;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    vec,
};
use num_bigint::BigInt;
use serde_json::Number;
/// {"method":"isPrime","number":123}\n
#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    method: String,
    number: Number,
}

/// {"method": "isPrime", "prime": true}\n
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IsPrimeRpcResponse {
    method: String,
    prime: bool,
}

fn is_prime(num: Number) -> bool {
    match BigInt::from_str(num.to_string().as_str()) {
        Err(_) => false,
        Ok(num) if num < BigInt::ZERO + 2 => false,
        Ok(num) => {
            let max = num.sqrt();
            let mut x: BigInt = BigInt::ZERO + 2;
            while x <= max {
                let modulo = num.clone() % x.clone();
                // println!("x:{:?}\tmax:{:?}\t{:?}", x, max, modulo);
                if modulo == BigInt::ZERO {
                    return false;
                }
                x += 1;
            }
            // println!("x:{:?}\nmax:{:?}", x, max);
            true
        }
    }
}

struct MessageReader {
    stream: TcpStream,
    delimiter: u8,
    buf: [u8; 128],
    queue: VecDeque<Vec<u8>>,
}

impl MessageReader {
    fn new(stream: TcpStream, delimiter: u8) -> MessageReader {
        MessageReader {
            stream,
            delimiter,
            buf: [0; 128],
            queue: VecDeque::from([vec![]]),
        }
    }

    fn split_buf(&self, buf: &[u8]) -> Result<VecDeque<Vec<u8>>, Box<dyn Error>> {
        // index of delimiter
        let mut split = VecDeque::<Vec<u8>>::from([vec![]]);
        for b in buf {
            split.back_mut().expect("Should not be empty").push(*b);
            if *b == self.delimiter {
                split.push_back(vec![]);
                continue;
            }
        }
        Ok(split)
    }

    fn pop_if_ready(&mut self) -> Option<Vec<u8>> {
        match self.queue.front()?.last() {
            Some(last) if last == &self.delimiter => self.queue.pop_front(),
            _ => None,
        }
    }

    fn serialize(msg: Vec<u8>) -> Result<RpcRequest, Box<dyn Error>> {
        match serde_json::from_slice::<RpcRequest>(&msg) {
            Ok(parsed) => {
                println!("Parsed: {:?}", parsed);
                Ok(parsed)
            }
            Err(e) => {
                println!("ParseErr: {:?}", e);
                println!("{}", String::from_utf8_lossy(&msg));
                Err(Box::new(e))
            }
        }
    }

    fn deserialize(msg: IsPrimeRpcResponse) -> Result<Vec<u8>, Box<dyn Error>> {
        match serde_json::to_vec(&msg) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn next_message(&mut self) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        loop {
            // if msg in queue, pop+return
            if let Some(msg) = self.pop_if_ready() {
                return Ok(Some(msg));
            }
            // read data appending to the message queue
            loop {
                let read = self.stream.read(&mut self.buf)?;
                if read == 0 {
                    self.stream.shutdown(Shutdown::Both)?;
                    return Ok(None);
                }

                match &self.buf[0..read] {
                    chunk if !chunk.contains(&self.delimiter) => {
                        println!("Buffer: {:?}", chunk);
                        self.queue
                            .front_mut()
                            .expect("Front should not be empty")
                            .extend_from_slice(chunk);
                    }
                    chunks => {
                        println!("Buffer: {:?}", chunks);
                        let split = &mut self.split_buf(chunks)?;
                        let mut front = split.pop_front().expect("Front should not be empty");
                        self.queue
                            .front_mut()
                            .expect("Front should not be empty")
                            .append(&mut front);
                        self.queue.append(split);
                        break;
                    }
                }
            }
        }
    }
}

fn handle_request(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut reader = MessageReader::new(stream, 10);
    loop {
        match reader.next_message() {
            Ok(None) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
            Ok(Some(msg)) => match MessageReader::serialize(msg.clone()) {
                Ok(serialized) if serialized.method == "isPrime" => {
                    let resp = IsPrimeRpcResponse {
                        method: "isPrime".to_owned(),
                        prime: is_prime(serialized.number),
                    };
                    let mut deserialized = MessageReader::deserialize(resp)?;
                    deserialized.push(reader.delimiter);
                    reader.stream.write_all(&deserialized)?;
                }
                _ => {
                    println!("BadReq Response: {:?}", msg);
                    reader.stream.write_all(msg.as_slice())?;
                }
            },
        }
    }
}
fn main() {
    tcp_accept_and_spawn(([0, 0, 0, 0], 6969).into(), handle_request).unwrap();
}
