use std::io::BufRead;

const INPUT_FILE: &str = "resources/day1.txt";

pub fn part1() {
    let file = std::fs::File::open(INPUT_FILE).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut increase_count = 0;
    let mut previous: Option<u16> = None;

    for line in reader.lines() {
        let current_num = line.unwrap().parse::<u16>().unwrap();
        
        if let Some(previous) = previous {
            if current_num > previous {
                increase_count += 1;
            }
        }
        
        previous = Some(current_num);
    }

    println!("increase amount: {}", increase_count);
}

pub fn part2() {
    let file = std::fs::File::open(INPUT_FILE).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut data = vec![];

    for line in reader.lines() {
        let num = line.unwrap().parse::<u16>().unwrap();
        data.push(num);
    }

    let mut increase_count = 0;
    let mut previous_sum: Option<u16> = None;

    for index in 0..(data.len() - 1 - 1) {
        let sum = data[index] + data[index + 1] + data[index + 2];

        if let Some(previous_sum) = previous_sum {
            if sum > previous_sum {
                increase_count += 1;
            }
        }
        
        previous_sum = Some(sum);
    }

    println!("increase amount: {:?}", increase_count);
}