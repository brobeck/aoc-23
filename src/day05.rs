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

#[derive(Debug)]
struct RangeRule {
    dest_start: i64,
    src_start: i64,
    range_len: i64,
}

fn within_range(val_to_check: i64, src_start: i64, range_len: i64) -> bool {
    src_start <= val_to_check && val_to_check <= (src_start + range_len)
}

fn map_given_range(val_to_map: i64, dest_start: i64, src_start: i64) -> i64 {
    let val_to_map_dist_from_start = val_to_map - src_start;
    dest_start + val_to_map_dist_from_start
}

fn map_given_rules(val_to_map: i64, range_rules: &Vec<RangeRule>) -> i64 {
    for range_rule in range_rules.iter() {
        if within_range(val_to_map, range_rule.src_start, range_rule.range_len) {
            return map_given_range(val_to_map, range_rule.dest_start, range_rule.src_start);
        }
    }
    val_to_map
}

fn map_to_vec_of_rules(map: &str) -> Vec<RangeRule> {
    let (_title, rules) = map.split_once('\n').unwrap();
    rules
        .lines()
        .map(|rule| {
            let rule_numbers: Vec<i64> = rule
                .split(' ')
                .map(|rule_number| rule_number.parse().unwrap())
                .collect();

            RangeRule {
                dest_start: rule_numbers[0],
                src_start: rule_numbers[1],
                range_len: rule_numbers[2],
            }
        })
        .collect()
}

fn solve_task_1() {
    let input = fs::read_to_string("input/input_5.txt").unwrap();
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let all_range_rules: Vec<Vec<RangeRule>> =
        maps.split("\n\n").map(map_to_vec_of_rules).collect();

    let (_seed_title, seeds) = seeds.split_once(' ').unwrap();

    let ans = seeds.split(' ').map(
        |seed|
        {
            let seed: i64 = seed.parse().unwrap();
            all_range_rules.iter().fold(seed, map_given_rules)
        }
    ).min().unwrap();

    println!("{ans}")
}

fn solve_task_2() {
    let _input = fs::read_to_string("input_4.txt").unwrap();
    eprintln!("This task is not implemented yet!");
    std::process::exit(1);
}
