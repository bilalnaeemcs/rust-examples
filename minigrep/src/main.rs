use std::env;
use std::process::exit;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("error : {err}");
        exit(1);
    });

    let _ = run(config).unwrap_or_else(|_| {
        //println!("Could not get the files {error}");
        exit(1);
    });
    //print!("ret : {ret}");
}
