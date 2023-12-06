use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("Cannot open file!");
    let reader = BufReader::new(file);

    let number_map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let answer = reader.lines().fold(0, |acc, line| -> i32 {
        let line = line.unwrap_or(String::from(""));
        let mut num = 0;
        'outer: for i in 0..line.len() {
            for j in 1..=cmp::min(5, line.len() - i) {
                if j == 2 {
                    continue;
                }
                let number_key = &line[i..(i + j)];
                if number_map.contains_key(&number_key) {
                    num = number_map[&number_key];
                    break 'outer;
                }
            }
        }
        'outer: for i in (0..line.len()).rev() {
            for j in 1..=cmp::min(5, line.len() - i) {
                if j == 2 {
                    continue;
                }
                let number_key = &line[i..(i + j)];
                if number_map.contains_key(&number_key) {
                    num = (num * 10) + number_map[&number_key];
                    break 'outer;
                }
            }
        }
        return acc + num; 
    });

    println!("{}", &answer);
}
