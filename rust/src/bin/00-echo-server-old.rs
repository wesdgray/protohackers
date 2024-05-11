use std::io::Write;
use std::net::TcpListener;
use std::net::Shutdown;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7").unwrap();
    // let (stream, socket) = listener.accept().unwrap();
    let incoming = listener.incoming();
    println!("{:?}", listener);
    println!("{:?}", incoming);
    for i in incoming {
        let mut t = i.unwrap();
        println!("{:?}", t);
        let mut buf: [u8; 1024] = [0; 1024];
        loop {
            let read = t.read(&mut buf).unwrap();
            if let Ok(s) = String::from_utf8(buf.into()) {
                println!("{}", s);
            }
            t.write_all(&buf[..read]).unwrap();
            buf = [0; 1024];
            if read == 0 {
                t.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}