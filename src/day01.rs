use std::fs;

pub fn run(task: &str) {
    match task {
        "1" => solve_task_1(),
        "2" => solve_task_2(),
        _ => {
            eprintln!("Unknown task: {}", task);
            std::process::exit(1);
        }
    }
}

fn solve_task_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let ans: u32 = input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first_digit * 10 + last_digit
        })
        .sum();

    println!("{ans}")
}

fn solve_task_2() {
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let input = fs::read_to_string("input.txt").unwrap();

    let ans = input
        .lines()
        .map(|line| {
            let mut min_first_index = None;
            let mut first_value = 0;
            let mut max_last_index = None;
            let mut last_value = 0;

            for (word, value) in words {
                if let Some(index) = line.find(word) {
                    if min_first_index.map_or(true, |x| index < x) {
                        min_first_index = Some(index);
                        first_value = value;
                    }
                }

                if let Some(index) = line.rfind(word) {
                    if max_last_index.map_or(true, |x| x < index) {
                        max_last_index = Some(index);
                        last_value = value;
                    }
                }
            }

            first_value * 10 + last_value
        })
        .sum::<i32>();

    println!("{ans}")
}
