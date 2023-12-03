// https://adventofcode.com/2023/day/2
use mgk_aoc::*;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    // println!("Part one: {}", part_one("./input_example.txt"));
    println!("Part one: {}", part_one("./input.txt"));
    // println!("Part two: {}", part_two("./input_example.txt"));
    println!("Part two: {}", part_two("./input.txt"));
}

fn part_one<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut sum: u32 = 0;
    let color_limit: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                sum += check_cube(line.as_str(), color_limit.borrow().clone())
            }
        }
    }

    sum
}

fn part_two<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut sum: u32 = 0;
    let color_limit: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                sum += check_cube_two(line.as_str(), color_limit.borrow().clone())
            }
        }
    }

    sum
}

fn check_cube(input: &str, color_limit: HashMap<&str, u32>) -> u32 {
    // unimplemented!();

    let mut valid: bool = true;
    let re_game_id_str = format!(r"Game\s(\d+)");
    let re_game_id = Regex::new(re_game_id_str.as_str()).unwrap();

    // println!("{input}");
    // println!("{re_game_id_str}");

    let game_id = re_game_id.captures(input).unwrap().get(1).unwrap().as_str();

    for (color, limit) in color_limit.into_iter() {
        let re_cube_str = format!("(\\d+)\\s({color})");
        let re_cube = Regex::new(re_cube_str.as_str()).unwrap();

        // println!("{re_cube_str}");

        for (_, [re_count, re_color]) in re_cube.captures_iter(input).map(|c| c.extract()) {
            // println!("{re_color}: {re_count} with limit {limit}");
            if re_count.parse::<u32>().unwrap() <= limit {
                // println!("{re_color}: {re_count} with limit {limit}");
                // println!("{re_color} is valid!")
            } else {
                // println!("{re_color} {re_count} with limit {limit} = invalid!");
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }
    }

    if valid {
        game_id.parse::<u32>().unwrap()
    } else {
        0
    }
}

fn check_cube_two(input: &str, color_limit: HashMap<&str, u32>) -> u32 {
    let mut red_min: u32 = 0;
    let mut green_min: u32 = 0;
    let mut blue_min: u32 = 0;

    for (color, limit) in color_limit.into_iter() {
        let re_cube_str = format!("(\\d+)\\s({color})");
        let re_cube = Regex::new(re_cube_str.as_str()).unwrap();
        let mut current_min: &mut u32 = &mut red_min;

        match color {
            "red" => current_min = &mut red_min,
            "green" => current_min = &mut green_min,
            "blue" => current_min = &mut blue_min,
            _ => (),
        }

        // println!("{re_cube_str}");

        for (_, [re_count, re_color]) in re_cube.captures_iter(input).map(|c| c.extract()) {
            // println!("{re_color}: {re_count} with limit {limit}");
            let re_count_int = re_count.parse::<u32>().unwrap();
            if re_count_int >= *current_min {
                // println!("{re_color}: {re_count} with limit {limit}");
                // println!("{re_color} is valid!")
                *current_min = re_count_int;
            }
        }
    }

    // println!("red_min: {red_min}");
    // println!("green_min: {green_min}");
    // println!("blue_min: {blue_min}");

    red_min * green_min * blue_min
}

// ref
// - https://docs.rs/regex/latest/regex/index.html
