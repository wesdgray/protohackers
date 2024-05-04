use std::{
    io::{Error, Read, Write},
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    thread,
};

pub fn tcp_accept_and_spawn(
    bind_addr: SocketAddr,
    callback: fn(TcpStream) -> Result<(), Error>,
) -> Result<(), Error> {
    let listener = TcpListener::bind(bind_addr)?;
    println!("Listening: {:?}", listener);
    for conn in listener.incoming() {
        match conn {
            Err(conn) => {
                println!("{:?}", conn.raw_os_error().unwrap());
            }
            Ok(conn) => {
                println!("Spawning Thread for: {:?}", conn);
                thread::spawn(move || {
                    callback(conn).unwrap();
                });
            }
        }
    }
    Ok(())
}

pub fn echo_server(mut tcp_stream: TcpStream) -> Result<(), Error> {
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let read = tcp_stream.read(&mut buf).unwrap();
        if let Ok(s) = String::from_utf8(buf.into()) {
            println!("{}", s);
        }
        tcp_stream.write_all(&buf[..read]).unwrap();
        if read == 0 {
            tcp_stream.shutdown(Shutdown::Both).unwrap();
            println!("Shutting down: {:?}", tcp_stream);
            break;
        }
    }
    Ok(())
}
