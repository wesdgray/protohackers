use std::{io::Error, net::TcpStream};

use protohackers::tcp_accept_and_spawn;

fn is_prime(tcpstream: TcpStream) -> Result<(), Error> {
    Ok(())
}
fn main() {
    let _ = tcp_accept_and_spawn(([0,0,0,0], 7).into(), is_prime);
}