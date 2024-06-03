mod connect;
mod server;

use std::env;
use connect::*;
use crate::server::run_server;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        println!("bind to {}:{}", &args[1].to_string(), &args[2].to_string());
        let _ = run_server( &args[1].to_string(), &args[2].to_string() ).unwrap();
        return
    } else {
        return
    }

}
