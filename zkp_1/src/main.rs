use clap::{arg, Command};

const EXPECTED: u64 = 96;

fn main() {
    let matches = Command::new("ZKP_1")
        .version("1.0")
        .about("My first zkp")
        .arg(arg!(--a <VALUE>).required(true))
        .arg(arg!(--b <VALUE>).required(true))
        .get_matches();

    verify(matches.get_one::<String>("a").expect("required a").parse::<u64>().expect("failed parse a") + matches.get_one::<String>("b").expect("required b").parse::<u64>().expect("failed parse b"));
}


fn verify(n: u64) {
    if n != EXPECTED {
        panic!("wrong proof");
    } else {
        println!("success");
    }
}
