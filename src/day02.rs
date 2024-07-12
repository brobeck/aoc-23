use std::{cmp, fs};

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

fn solve_task_2() {
    let input = fs::read_to_string("inputs/input_2.txt").unwrap();

    let ans: i32 = input
        .lines()
        .map(|line| {
            let (_, game_plays) = line.split_once(": ").unwrap();

            let mut max_blue = 0;
            let mut max_red = 0;
            let mut max_green = 0;

            for play in game_plays.split("; ") {
                for draw in play.split(", ") {
                    let (n, color) = draw.split_once(' ').unwrap();

                    let n = n.parse::<i32>().unwrap();

                    match color {
                        "blue" => max_blue = cmp::max(max_blue, n),
                        "red" => max_red = cmp::max(max_red, n),
                        "green" => max_green = cmp::max(max_green, n),
                        unknown => panic!("Unknown color: {}", unknown),
                    };
                }
            }

            max_blue * max_red * max_green
        })
        .sum();

    println!("{ans}")
}

fn _possible_game(game_plays: &str) -> bool {
    for play in game_plays.split("; ") {
        for draw in play.split(", ") {
            let (n, color) = draw.split_once(' ').unwrap();

            let n = n.parse::<i32>().unwrap();

            let possible_draw = match color {
                "blue" => n <= 14,
                "red" => n <= 12,
                "green" => n <= 13,
                unknown => panic!("Unknown color: {}", unknown),
            };

            if !possible_draw {
                return false;
            };
        }
    }
    true
}

fn _count_for_game(line: &str) -> i32 {
    let (game_name, game_plays) = line.split_once(": ").unwrap();

    if _possible_game(game_plays) {
        let (_, game_n) = game_name.split_once(' ').unwrap();
        game_n.parse().unwrap()
    } else {
        0
    }
}

fn solve_task_1() {
    let input = fs::read_to_string("inputs/input_2.txt").unwrap();

    let ans: i32 = input.lines().map(_count_for_game).sum();

    println!("{ans}")
}
