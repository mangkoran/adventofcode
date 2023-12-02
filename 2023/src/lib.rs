use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// pub fn read_lines(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();
//
//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string())
//     }
//
//     result
// }

// pub fn read_lines(filename: &str) -> Vec<String> {
//     read_to_string(filename)
//         .unwrap() // panic on possible file-reading errors
//         .lines() // split the string into an iterator of string slices
//         .map(String::from) // make each slice into a string
//         .collect() // gather them together into a vector
// }

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
