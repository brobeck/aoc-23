use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

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

#[derive(Hash, Eq, PartialEq, Debug)]
struct StarLocation {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct StarTest {
    n_neigh_numbers: i32,
    tally_neigh_numbers: i32,
}

impl StarTest {
    fn new() -> Self {
        StarTest {
            n_neigh_numbers: 0,
            tally_neigh_numbers: 1,
        }
    }
}

fn add_adjacent_stars(
    row_center: usize,
    col_center: usize,
    grid: &[[char; 140]; 140],
    seen_stars: &mut HashSet<StarLocation>,
) {
    let min_row = row_center.saturating_sub(1);
    let max_row = (row_center + 1).min(139);
    let min_col = col_center.saturating_sub(1);
    let max_col = (col_center + 1).min(139);

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if grid[row][col] == '*' {
                seen_stars.insert(StarLocation { row, col });
            }
        }
    }
}

fn solve_task_2() {
    let input = fs::read_to_string("input_3.txt").unwrap();

    const N_ROWS: usize = 140;
    const N_COLS: usize = 140;
    let mut grid: [[char; N_COLS]; N_ROWS] = [[' '; N_COLS]; N_ROWS];

    for (row_index, row) in input.lines().enumerate() {
        for (col_index, char_val) in row.chars().enumerate() {
            grid[row_index][col_index] = char_val
        }
    }

    let mut all_stars: HashMap<StarLocation, StarTest> = HashMap::new();
    let mut current_number = 0;

    for (row_index, row) in input.lines().enumerate() {
        let mut char_iter = row.match_indices(char::is_numeric).peekable();
        let mut seen_stars = HashSet::new();

        while let Some((col_index, number)) = char_iter.next() {
            let number: i32 = number.parse().expect("Failed to parse number");
            current_number = current_number * 10 + number;
            add_adjacent_stars(row_index, col_index, &grid, &mut seen_stars);

            if char_iter
                .peek()
                .map_or(true, |(next_index, _)| *next_index != col_index + 1)
            {
                for star in seen_stars.drain() {
                    let star_test = all_stars.entry(star).or_insert_with(StarTest::new);
                    star_test.n_neigh_numbers += 1;
                    star_test.tally_neigh_numbers *= current_number;
                }

                current_number = 0;
            }
        }
    }

    let sum_gears: i32 = all_stars
        .values()
        .filter(|star_test| star_test.n_neigh_numbers == 2)
        .map(|star_test| star_test.tally_neigh_numbers)
        .sum();

    println!("{sum_gears}");
}

fn is_symbol(c: char) -> bool {
    !(c.is_numeric() || c == '.')
}

fn adjacent_symbol(row: usize, col: usize, grid: &[[char; 140]; 140]) -> bool {
    let min_row = row.saturating_sub(1);
    let max_row = (row + 1).min(139);
    let min_col = col.saturating_sub(1);
    let max_col = (col + 1).min(139);

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if (r != row || c != col) && is_symbol(grid[r][c]) {
                return true;
            }
        }
    }
    false
}

fn solve_task_1() {
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
    let mut current_number = 0;
    let mut current_is_part = false;

    for (row_index, row) in input.lines().enumerate() {
        let mut char_iter = row.match_indices(char::is_numeric).peekable();

        while let Some((col_index, number)) = char_iter.next() {
            let number: i32 = number.parse().unwrap();
            current_number = current_number * 10 + number;
            current_is_part |= adjacent_symbol(row_index, col_index, &grid);

            if char_iter
                .peek()
                .map_or(true, |(next_index, _)| *next_index != col_index + 1)
            {
                if current_is_part {
                    current_sum += current_number;
                }

                current_number = 0;
                current_is_part = false;
            }
        }
    }

    println!("{}", current_sum)
}
