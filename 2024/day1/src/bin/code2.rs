use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let file_path = "src/bin/input.txt";

    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut a_value: Vec<i32> = Vec::new();
    let mut b_value: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                a_value.push(a);
                b_value.push(b);
            } else {
                eprintln!("Error parsing numbers on line: {}", line);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    let len_a = a_value.len();
    let mut reoccur = HashMap::new();
    for i in 0..len_a {
        let counter = reoccur.entry(b_value[i]).or_insert(0);
        *counter += 1;
    }
    let mut score = 0;
    for a in &a_value {
        for (key, value) in &reoccur {
            if a == key {
                score += a * value
            }
        }
    }

    print!("{:?}", score);

    Ok(())
}
