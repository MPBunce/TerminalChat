mod connect;
mod server;
mod client;

use std::env;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::server::run_server;

#[tokio::main]
async fn main() {

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let args: Vec<String> = env::args().collect();
    //println!("{}", &args.len());
    if args.len() != 4 {
        return
    } else {
        let cmd = args[1].clone();
        //println!("{}", &cmd);
        match cmd.to_lowercase().as_str() {
            "connect" => {
                println!("Connecting to {}:{}", &args[2].to_string(), &args[3].to_string());
                // runtime.block_on( async {
                //
                // })
                client::client().await.unwrap();
            },
            "serve" => {
                println!("Serving on {}:{}", &args[2].to_string(), &args[3].to_string());
                // runtime.block_on( async {
                //
                // })
                client::connect_server().await.unwrap();
            },
            _ => {
                println!("No Match");
                return
            }
        }
    }


}

pub async fn run_async() -> Result<(), String> {
    let mut stdin = BufReader::new(io::stdin());
    let mut line = String::new();

    while let Ok(bytes_read) = stdin.read_line(&mut line).await {
        if bytes_read == 0 {
            break;
        }
        println!("Read: {}", line.trim());
        line.clear();
    }

    Ok(())
}