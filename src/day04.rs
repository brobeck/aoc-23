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
    let _input = fs::read_to_string("inputs/input_4.txt").unwrap();
    eprintln!("This task is not implemented yet!");
    std::process::exit(1);
}

fn solve_task_2() {
    let _input = fs::read_to_string("inputs/input_4.txt").unwrap();
    eprintln!("This task is not implemented yet!");
    std::process::exit(1);
}