use std::io::{Error, Read, Write};
use std::net::TcpStream;
use protohackers::tcp_accept_and_spawn;

fn echo(mut tcp_stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 64];
    while let Ok(count) = tcp_stream.read(&mut buf) {
        if count == 0 {
            break;
        }
        tcp_stream.write_all(&buf[..count])?;
    }
    tcp_stream.flush()?;
    Ok(())
}

fn main() {
    tcp_accept_and_spawn(([0,0,0,0], 7).into(), echo).unwrap();
}
