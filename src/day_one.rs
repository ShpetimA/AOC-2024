use std::{collections::HashMap, fs::read_to_string};

pub fn part_two() {
    let file = read_to_string("locations.txt").expect("File doesn't exist");
    let mut col_one: Vec<i32> = vec![];
    let mut col_two: HashMap<i32, i32> = HashMap::new();

    file.lines().for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(idx, num)| {
                if let Ok(num) = num.trim().parse::<i32>() {
                    if idx % 2 == 0 {
                        col_two.entry(num).and_modify(|x| *x += 1).or_insert(1);
                    } else {
                        col_one.push(num);
                    }
                }
            })
    });

    col_one.sort();

    let mut sum = 0;

    for num in col_one {
        col_two.entry(num).and_modify(|x| sum += *x * num);
    }

    println!("SUM PART 2: {sum}");
}

pub fn part_one() {
    let file = read_to_string("locations.txt").expect("File doesn't exist");
    let (mut col_one, mut col_two): (Vec<i32>, Vec<i32>) = file
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    col_one.sort();
    col_two.sort();

    let mut sum = 0;

    for (x, y) in col_one.iter().zip(col_two.iter()) {
        sum += (x - y).abs();
    }

    println!("SUM {}", sum);
}
