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

fn extract_mul(input: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    re.captures_iter(input)
        .filter_map(|caps| caps.get(0).map(|matched| matched.as_str().to_string())) // Get only full matches
        .collect()
}

fn mul_digit_extract(input: &str) -> Vec<(i32, i32)> {
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
                let regx_mul = extract_mul(&line);
                let mut enabled = true;
                let mut sum = 0;

                for valid in regx_mul {
                    if valid == "don't()" {
                        enabled = false;
                    } else if valid == "do()" {
                        enabled = true;
                    }
                    if enabled {
                        let ext_num = mul_digit_extract(&valid);
                        for (v, m) in ext_num {
                            let product = v * m;
                            sum += product;
                        }
                    }
                }
                total_sum += sum;
            }
            println!("Total Sum: {:?}", total_sum);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
