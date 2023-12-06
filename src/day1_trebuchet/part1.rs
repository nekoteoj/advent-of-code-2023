use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("Cannot open file!");
    let reader = BufReader::new(file);

    let answer = reader.lines().fold(0, |acc, line| -> i32 {
        let num_chars: Vec<char> = line
            .unwrap_or(String::from(""))
            .chars()
            .filter(|x| x.is_numeric())
            .collect();
        let num_string = format!(
            "{}{}",
            num_chars.first().unwrap_or(&'0'),
            num_chars.last().unwrap_or(&'0')
        );
        return acc + num_string.parse().unwrap_or(0);
    });

    println!("{}", &answer);
}
