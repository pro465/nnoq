use nnoq::*;
use std::fs;

fn main() {
    let prog = fs::read_to_string(
        fs::canonicalize(std::env::args().nth(1).unwrap_or_else(|| help()))
            .expect("could not canonicalize argument"),
    )
    .expect("could not read file");

    let result = verify(prog);
    if let Err(e) = result {
        e.report();
        std::process::exit(-1);
    }
    println!("verified");
}

fn help() -> ! {
    println!(
        "usage: {} <filename>",
        std::env::current_exe()
            .unwrap_or_else(|_| "nnoq".into())
            .display()
    );
    std::process::exit(-1);
}
