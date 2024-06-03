use std::io::{Read, stdout, Write};
use std::net::TcpListener;

pub fn run_server(host: &String, port: &String) -> Result<(), String> {
    let addr = format!("{}:{}",host, port);
    println!("{}", addr.clone());
    let mut listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Connection Accepted");
                let mut buf = [0; 128];
                let mut read_bytes = 0;
                while read_bytes == 0 {
                    read_bytes = s.read(&mut buf).map_err(|_| "failed to read from socket")?;
                    println!("Received bytes {}", read_bytes)
                }
                stdout().write(&buf[0..read_bytes] ).map_err(|_| "failed to write to stdout")?;
                stdout().flush().unwrap();
            } Err(e) => {
                println!("Error while accepting connection - {}", e)
            }
        }
    }

    Ok(())
}