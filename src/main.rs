use std::env;

use openssl::base64;
use openssl::sha::Sha256;

fn main() {
    let args: Vec<String> = env::args().collect();
    let method_name = args[0].as_str();
    match method_name {
        "say" => say(args[1].as_str()),
        "to_sha256" => to_sha256(args[1].as_str()),
        _ => std::process::exit(1),
    };
}


fn say(name: &str) {
    eprintln!("{} say1: Hello World", name);
}

fn to_sha256(value: &str) {
    let vec = base64::decode_block(value).unwrap();
    let mut sha256 = Sha256::new();
    sha256.update(vec.as_slice());
    eprintln!("{}", base64::encode_block(sha256.finish().as_slice()));
}