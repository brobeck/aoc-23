use std::fs;
use std::cmp;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    _day_3_1();    
    let duration = start_time.elapsed();

    println!("Execution time: {:?}", duration);
}

fn _adjacent_symbol(row: usize, col: usize, grid: &[[char; 140]; 140]) -> bool {
    for r in [cmp::max(row, 1) - 1, row, cmp::min(row, 138) + 1] {
        for c in [cmp::max(col, 1) - 1, col, cmp::min(col, 138) + 1]  {
            let grid_value = grid[r][c];
            if !grid_value.is_numeric() && grid_value != '.' {
                return true;
            } 
        }    
    };
    false
}

fn _day_3_1() {
    let input = fs::read_to_string("input_3.txt").unwrap();

    const N_ROWS: usize = 140;
    const N_COLS: usize = 140;
    let mut grid: [[char; 140]; 140] = [[' '; N_COLS]; N_ROWS];

    for (row_index, row) in input.lines().enumerate() {
        for (col_index, char_val) in row.chars().enumerate() {
            grid[row_index][col_index] = char_val
        }
    }

    let mut current_sum = 0;

    for (row_index, row) in input.lines().enumerate() {
        let mut current_number = 0;
        let mut last_index = 0;
        let mut current_is_part = false;

        for (number_index, number) in row.match_indices(char::is_numeric) {
            if number_index == last_index + 1 || last_index == 0 {
                current_number = current_number * 10 + number.parse::<i32>().unwrap();
                current_is_part |= _adjacent_symbol(row_index, number_index, &grid);
            } else {
                if current_is_part {
                    current_sum += current_number
                }
                current_number = number.parse::<i32>().unwrap();
                current_is_part = _adjacent_symbol(row_index, number_index, &grid);
            }
            last_index = number_index;
        }

        if current_is_part {
            current_sum += current_number
        }
    }

    println!("{current_sum}")
}

fn _day_2_2() {
    let input = fs::read_to_string("input_2.txt").unwrap();

    let ans: i32 = input.lines().map(|line| {
        let(_, game_plays) = line.split_once(": ").unwrap();

        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for play in game_plays.split("; ") {
            for draw in play.split(", ") {
                let (n, color) = draw.split_once(' ').unwrap();
                
                let n = n.parse::<i32>().unwrap();
    
                match color {
                    "blue"  => max_blue = cmp::max(max_blue, n),
                    "red"   => max_red = cmp::max(max_red, n),
                    "green" => max_green = cmp::max(max_green, n),
                    unknown => panic!("Unknown color: {}", unknown)
                };
            }
        }
        
        max_blue * max_red * max_green
    }).sum();

    println!("{ans}")
}

fn _possible_game(game_plays: &str) -> bool {
    for play in game_plays.split("; ") {
        for draw in play.split(", ") {
            let (n, color) = draw.split_once(' ').unwrap();
            
            let n = n.parse::<i32>().unwrap();

            let possible_draw = match color {
                "blue"  => n <= 14,
                "red"   => n <= 12,
                "green" => n <= 13,
                unknown => panic!("Unknown color: {}", unknown)
            };

            if !possible_draw {
                return false; 
            };
        }
    }
    true
}

fn _count_for_game(line: &str) -> i32 {
    let(game_name, game_plays) = line.split_once(": ").unwrap();

    if _possible_game(game_plays) {
        let (_, game_n) = game_name.split_once(' ').unwrap();
        game_n.parse().unwrap()
    } else {
        0
    }
}

fn _day_2_1() {
    let input = fs::read_to_string("input_2.txt").unwrap();

    let ans: i32 = input.lines().map(_count_for_game).sum();

    println!("{ans}")
}   

fn _day_1_1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let ans: u32 = input.lines().map(|line| {
        let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        first_digit * 10 + last_digit
    }).sum();

    println!("{ans}")
}

fn _day_1_2() {
    let words = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ];

    let input = fs::read_to_string("input.txt").unwrap();

    let ans = input.lines().map(|line| {
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
    }).sum::<i32>();

    println!("{ans}")
}