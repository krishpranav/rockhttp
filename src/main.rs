extern crate rockhttp_lib;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut address:String  = "localhost:8080".to_string();
}