use std::io::BufRead;

const INPUT_FILE: &str = "resources/day2.txt";

pub fn part1() {
    let file = std::fs::File::open(INPUT_FILE).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (command, amount) = {
            let split = line.split(' ').collect::<Vec<&str>>();
            (split[0], split[1].parse::<u32>().unwrap())
        };

        match command {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => {}
        }
    }

    println!("horizontal position: {}", horizontal);
    println!("depth: {}", depth);
    println!("horizontal position * depth: {}", horizontal * depth);
}

pub fn part2() {
    let file = std::fs::File::open(INPUT_FILE).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (command, amount) = {
            let split = line.split(' ').collect::<Vec<&str>>();
            (split[0], split[1].parse::<u32>().unwrap())
        };

        match command {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            _ => {}
        }
    }

    println!("horizontal position: {}", horizontal);
    println!("depth: {}", depth);
    println!("aim: {}", aim);
    println!("horizontal position * depth: {}", horizontal * depth);
}
