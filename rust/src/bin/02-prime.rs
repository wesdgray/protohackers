use std::{io::{Error, Read, Write}, net::{Shutdown, TcpStream}, ops::Index, thread::sleep, time::Duration, vec};
use protohackers::tcp_accept_and_spawn;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_slice;
use primes::is_prime;
/// {"method":"isPrime","number":123}\n
#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    method: String,
    number: i128
}

/// {"method": "isPrime", "prime": true}\n
#[derive(Serialize, Deserialize, Debug, Clone)]
struct IsPrimeRpcResponse {
    method: String,
    prime: bool
}

fn is_primez(num: i128) -> bool {
    if num < 0 {
        return false;
    }
    is_prime(num as u64)
}

struct MessageReader {
    stream: TcpStream,
    delimiter: u8,
    buf: [u8;128],
    req: Vec<u8>
}


impl MessageReader {
    fn new(stream: TcpStream, delimiter: u8) -> MessageReader {
        MessageReader{
            stream,
            delimiter,
            buf: [0; 128],
            req: vec![],
        }
    }

    fn next_message(&mut self) -> Result<Option<Vec<u8>>, Error> {
        loop {
            let read = self.stream.read(&mut self.buf)?;
            if read == 0 {
                return Ok(None);
            }

            if let Some((index, _)) = self.buf.iter().enumerate().find(|(_,v)| **v == self.delimiter) {
                if index - 1 == self.req.len(){
                    self.req.clear();
                } else {
                    self.req = self.buf[index+1..].to_vec();
                }
                let left = &self.buf[0..index];
                return Ok(Some(left.to_vec()));
            }
        }
    }
}

fn handle_request(stream: TcpStream) -> Result<(), Error> {
    let mut reader = MessageReader::new(stream, 10);
    while let Some(msg) = reader.next_message().transpose() {
        let msg = match msg {
            Err(e) => return Err(e),
            Ok(m) => m
        };
        println!("Message: {}", String::from_utf8_lossy(&msg));
        match serde_json::from_slice::<RpcRequest>(&msg) {
            Ok(parsed) if parsed.method == "isPrime" => {
                println!("Parsed: {:?}", parsed);
                let resp = IsPrimeRpcResponse{method: "isPrime".to_owned(), prime: is_primez(parsed.number)};
                reader.stream.write_all(&serde_json::to_vec(&resp)?)?
            },
            err => {
                println!("ParseErr: {:?}", err);
                reader.stream.write_all(&msg)?;
            }
        }
        sleep(Duration::from_millis(200));
    }
    println!("Exited Abnormally");
    Ok(())
}
fn main() {
    tcp_accept_and_spawn(([0,0,0,0], 6969).into(), handle_request).unwrap();
}