mod connect;
use std::env;
use connect::*;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let b = run( &args[1].to_string(), &args[2].to_string() );
        return
    } else {
        return
    }

}
