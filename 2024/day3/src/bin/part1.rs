use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn read_file(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let output: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(output)
}

fn extract_mul(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .filter_map(|caps| {
            let first = caps[1].parse::<i32>().ok();
            let second = caps[2].parse::<i32>().ok();

            match (first, second) {
                (Some(f), Some(s)) => Some((f, s)),
                _ => None,
            }
        })
        .collect()
}

fn main() {
    match read_file("src/bin/input.txt") {
        Ok(lines) => {
            let mut total_sum = 0;
            for line in lines {
                let mut sum = 0;
                let regx_mul = extract_mul(&line);
                for (v, m) in regx_mul.iter() {
                    let product = v * m;
                    sum += product;
                }
                total_sum += sum;
            }
            println!("{:?}", total_sum);
        }
        Err(e) => {
            eprintln!("Erorr reading file: {}", e)
        }
    }
}
