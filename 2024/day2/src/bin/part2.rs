use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_file(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let output: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(output)
}

fn convert_int(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|part| (part.parse::<i32>()).ok()) // Ignore errors and collect valid results
        .collect()
}

fn is_report_safe(report_num: &[i32]) -> bool {
    let mut flag_dec = false;
    let mut flag_inc = false;

    for i in 1..report_num.len() {
        let diff = report_num[i] - report_num[i - 1];

        if diff > 0 {
            flag_inc = true;
        } else if diff < 0 {
            flag_dec = true;
        } else {
            return false;
        }

        if flag_inc && flag_dec {
            return false;
        }

        if diff > 3 || diff < -3 {
            return false;
        }
    }

    true
}

fn main() {
    match read_file("src/bin/input.txt") {
        Ok(lines) => {
            let mut counter = 0;
            for report in lines {
                let n_report = convert_int(&report);
                if is_report_safe(&n_report) {
                    counter += 1;
                } else {
                    for index in 0..n_report.len() {
                        let new_report: Vec<_> = n_report[..index]
                            .iter()
                            .chain(&n_report[index + 1..])
                            .cloned()
                            .collect();

                        if is_report_safe(&new_report) {
                            counter += 1;
                            break;
                        }
                    }
                }
            }
            println!("{:?}", counter);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e)
        }
    }
}
