// https://adventofcode.com/2023/day/1
use mgk_aoc::*;
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

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(input) = line {
                sum += get_calibration_value(input.as_str(), false)
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

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                sum += get_calibration_value(line.as_str(), true)
            }
        }
    }

    sum
}

fn get_calibration_value(line: &str, part_two: bool) -> u32 {
    let mut tenth: u32 = 0;
    let mut tenth_index: u32 = 0;
    let mut one: u32 = 0;
    let mut one_index: u32 = 0;

    let text_to_digit = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    if part_two {
        for (text, digit) in text_to_digit.into_iter() {
            if let Some(index) = line.find(text) {
                swap_digit(
                    &mut tenth,
                    &mut tenth_index,
                    &mut one,
                    &mut one_index,
                    index,
                    digit,
                )
            }

            if let Some(index) = line.rfind(text) {
                swap_digit(
                    &mut tenth,
                    &mut tenth_index,
                    &mut one,
                    &mut one_index,
                    index,
                    digit,
                )
            }
        }
    }

    for c in line.chars() {
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).unwrap();

            if let Some(index) = line.find(c) {
                swap_digit(
                    &mut tenth,
                    &mut tenth_index,
                    &mut one,
                    &mut one_index,
                    index,
                    digit,
                )
            }

            if let Some(index) = line.rfind(c) {
                swap_digit(
                    &mut tenth,
                    &mut tenth_index,
                    &mut one,
                    &mut one_index,
                    index,
                    digit,
                )
            }
        }
    }

    if one == 0 {
        one = tenth
    }

    // println!("{tenth}{one}");
    format!("{tenth}{one}").parse().unwrap()
}

fn swap_digit(
    tenth: &mut u32,
    tenth_index: &mut u32,
    one: &mut u32,
    one_index: &mut u32,
    index: usize,
    digit: u32,
) {
    if let Ok(y) = u32::try_from(index) {
        // println!("{tenth_index} - {one_index}");
        if *tenth == 0 || y < *tenth_index {
            *tenth_index = y;
            *tenth = digit
        }

        if y > *one_index {
            *one_index = y;
            *one = digit
        }
    }
}

// ref
// - https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#examples
