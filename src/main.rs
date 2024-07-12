mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        "1"  => day01::run(&args[2]),
        "2"  => day02::run(&args[2]),
        "3"  => day03::run(&args[2]), 
        "4"  => day04::run(&args[2]),
        "5"  => day05::run(&args[2]),
        "6"  => day06::run(&args[2]),
        "7"  => day07::run(&args[2]),
        "8"  => day08::run(&args[2]),
        "9"  => day09::run(&args[2]),
        "10" => day10::run(&args[2]),
        "11" => day11::run(&args[2]),
        "12" => day12::run(&args[2]),
        "13" => day13::run(&args[2]),
        "14" => day14::run(&args[2]),
        "15" => day15::run(&args[2]),
        "16" => day16::run(&args[2]),
        "17" => day17::run(&args[2]),
        "18" => day18::run(&args[2]),
        "19" => day19::run(&args[2]),
        "20" => day20::run(&args[2]),
        "21" => day21::run(&args[2]),
        "22" => day22::run(&args[2]),
        "23" => day23::run(&args[2]),
        "24" => day24::run(&args[2]),
        "25" => day25::run(&args[2]),
        _ => {
            eprintln!("Unknown day: {}", args[1]);
            std::process::exit(1);
        }
    }

    let duration = start_time.elapsed();

    println!("Execution time: {:?}", duration);
}
