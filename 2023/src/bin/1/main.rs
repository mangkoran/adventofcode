use mgk_aoc::*;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    println!("{}", part_one("./input.txt"));
    println!("{}", part_two("./input.txt"));
}

fn get_calibration_value(text: &str, part_two: bool) -> u32 {
    let mut tenth: u32 = 0;
    let mut one: u32 = 0;
    let mut tenth_index: u32 = 0;
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
        for (key, value) in text_to_digit.into_iter() {
            // tenth_index = u32::try_from(text.find(key).unwrap()).unwrap();
            if let Some(i) = text.find(key) {
                if let Ok(y) = u32::try_from(i) {
                    // println!("{y} - {key} - {value} - {tenth_index} - {one_index}");
                    if tenth == 0 || y < tenth_index {
                        tenth_index = y;
                        tenth = value;
                    }

                    if y > one_index {
                        one_index = y;
                        one = value;
                    }
                }
            }

            if let Some(i) = text.rfind(key) {
                if let Ok(y) = u32::try_from(i) {
                    // println!("{y} - {key} - {value} - {tenth_index} - {one_index}");
                    if tenth == 0 || y < tenth_index {
                        tenth_index = y;
                        tenth = value;
                    }

                    if y > one_index {
                        one_index = y;
                        one = value;
                    }
                }
            }
        }
    }

    for c in text.chars() {
        if c.is_ascii_digit() {
            if let Some(i) = text.find(c) {
                if let Ok(y) = u32::try_from(i) {
                    // println!("{y} - {key} - {value} - {tenth_index} - {one_index}");
                    if tenth == 0 || y < tenth_index {
                        tenth_index = y;
                        tenth = c.to_digit(10).unwrap()
                    }

                    if y > one_index {
                        one_index = y;
                        one = c.to_digit(10).unwrap()
                    }
                }
            }

            if let Some(i) = text.rfind(c) {
                if let Ok(y) = u32::try_from(i) {
                    // println!("{y} - {key} - {value} - {tenth_index} - {one_index}");
                    if tenth == 0 || y < tenth_index {
                        tenth_index = y;
                        tenth = c.to_digit(10).unwrap()
                    }

                    if y > one_index {
                        one_index = y;
                        one = c.to_digit(10).unwrap()
                    }
                }
            }
        }
    }

    if one == 0 {
        one = tenth
    }

    // println!("{tenth}{one}");
    format!("{tenth}{one}").parse().unwrap()
}

fn part_one<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut sum: u32 = 0;

    // get_calibration_value(input);

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

    // get_calibration_value(input);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(input) = line {
                sum += get_calibration_value(input.as_str(), true)
            }
        }
    }

    sum
}

// ref
// - https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#examples
