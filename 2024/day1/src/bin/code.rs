use std::{
    fs::File,
    io::{self, BufRead},
};

fn diff(v_1: Vec<i32>, v_2: Vec<i32>) -> Vec<i32> {
    return v_1.iter().zip(v_2).map(|(a, b)| (a - b).abs()).collect();
}

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
                a_value.sort();
                b_value.sort();
            } else {
                eprintln!("Error parsing numbers on line: {}", line);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    let d = diff(a_value, b_value);

    let sum: i32 = d.iter().sum();

    println!("{:?}", sum);

    Ok(())
}
