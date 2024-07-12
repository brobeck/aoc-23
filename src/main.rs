mod day01;
mod day02;
mod day03;

use std::env;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <day> <task>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "1" => day01::run(&args[2]),
        "2" => day02::run(&args[2]),
        "3" => day03::run(&args[2]),
        _ => {
            eprintln!("Unknown day: {}", args[1]);
            std::process::exit(1);
        }
    }

    let duration = start_time.elapsed();

    println!("Execution time: {:?}", duration);
}
