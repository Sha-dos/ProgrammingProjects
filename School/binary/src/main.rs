use binary::{BinaryToString, ToBinary};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 { panic!("Expected 2 arguments!"); }

    let str = &args[2];
    let flag: &str = args[1].as_str();

    match flag {
        "tostr" => println!("String value is: {}", str.binary_to_string()),
        "tobin" => println!("Binary value is: {}", str.to_binary()),
        &_ => println!("Error reading flag!")
    }

    //println!("Binary value is: {}", str.to_binary());
    //println!("String value is: {}", str.binary_to_string());
}
