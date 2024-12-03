use std::time::Instant;

use day_three::{part_one, part_two};

mod day_one;
mod day_two;
mod day_three;

fn main() {
    let now = Instant::now();
    part_one();
    let elapsed = now.elapsed().as_micros();
    println!("Time {elapsed}");

    let now = Instant::now();
    part_two();
    let elapsed = now.elapsed().as_micros();
    println!("Time 2 {elapsed}");
}
