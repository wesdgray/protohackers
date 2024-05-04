use std::{io::{Error, Read, Write}, net::{Shutdown, TcpStream}, thread::sleep, time::Duration, vec};
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
#[derive(Serialize, Deserialize, Debug)]
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
    fn new(&mut self, stream: TcpStream, delimiter: u8) {
        self.stream = stream;
        self.delimiter = delimiter;
        self.buf = [0; 128];
        self.req = vec![];
    }

    fn read(&self) -> u8 {
        todo!();
        //let read = self.stream.read(buf)
    }
}
fn handle_request(mut tcpstream: TcpStream) -> Result<(), Error> {
    // get the data
    let mut buf:[u8;256] = [0; 256];
    let mut req: Vec<u8> = vec![];
    while let Ok(read) = tcpstream.read(&mut buf) {
        // println!("{:?}", &buf);
        let data = &buf[..read];
        if read == 0 {
            tcpstream.shutdown(Shutdown::Both)?;
            return Ok(());
        }
        req.extend(data.iter());
        println!("Concatenating req: {}", String::from_utf8_lossy(&req));

        if let Some((newline_index, _)) = req.iter().enumerate().find(|(_index, value)| *value == &10) {
            let req_finished = Vec::from(&req[..=newline_index]);
            let s = String::from_utf8(req_finished.clone()).unwrap();
            println!("Finished request: {}", s.clone());
            req = req.iter().skip(newline_index+1).cloned().collect();
            println!("Resetting request to [{}]", String::from_utf8_lossy(&req));
            // parse the data
            let maybe_req: Result<RpcRequest, serde_json::Error> = from_slice(&req_finished);
            println!("Parsed result: {:?}", maybe_req);

            // malformed gets sent back, good requests are processed
            match maybe_req {
                Err(_) => {
                    tcpstream.write_all(req.as_slice())?;
                },
                Ok(parsed_req) => {
                    println!("Parsed request: {:?}", parsed_req);
                    let resp = IsPrimeRpcResponse{ method: "isPrime".to_owned(), prime: is_primez(parsed_req.number)};
                    let mut resp_vec = serde_json::to_vec(&resp)?;
                    println!("Deserialized json: {:?}", String::from_utf8(resp_vec.clone()));
                    resp_vec.append(&mut vec![10]);
                    tcpstream.write_all(resp_vec.as_slice())?;
                }
            }

        }
        sleep(Duration::from_millis(200));
    }
    Ok(())
}
fn main() {
    tcp_accept_and_spawn(([0,0,0,0], 7).into(), handle_request).unwrap();
}