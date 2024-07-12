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
    let input = fs::read_to_string("inputs/input_4.txt").unwrap();
    let mut card_number_array: [u32; 100] = [0; 100];
    let mut set_number_array: [usize; 10] = [0; 10];

    let ans: i32 = input
        .lines()
        .map(|line| {
            let (card_numbers, guess_numbers) = line.split_once('|').unwrap();

            for (index_set_array, card_number) in card_numbers
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<usize>().ok())
                .enumerate()
            {
                card_number_array[card_number] = 1;
                set_number_array[index_set_array] = card_number;
            }

            let correct_guesses = guess_numbers
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .map(|guess_number| card_number_array[guess_number])
                .sum::<u32>();

            for card_number in set_number_array {
                card_number_array[card_number] = 0;
            }

            match correct_guesses {
                0 => 0,
                x => 2_i32.pow(x - 1),
            }
        })
        .sum();

    println!("{ans}")
}

fn solve_task_2() {
    let _input = fs::read_to_string("inputs/input_4.txt").unwrap();
    eprintln!("This task is not implemented yet!");
    std::process::exit(1);
}
