use std::fmt::Error;
use std::io::Write;
use std::net::TcpStream;

pub fn run(host: &String, port: &String) -> Result<(), Error> {
    let addr = format!("{}:{}",host, port);
    let mut client = TcpStream::connect(addr).unwrap();
    let buf: &[u8] = "Hello from TcpStream!\n".as_bytes();
    let b = client.write(buf);
    Ok(())
}