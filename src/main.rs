mod day03{
    pub mod day03;
}
mod day04{
    pub mod day04;
}

use day04::day04::*;
use day03::day03::*;

fn main() {
    println!("--- Day 3 ---");
    day_three(r"src\day03\day03_input.txt");
    println!("--- Day 4 ---");
    day_four(r"src\day04\day04_input.txt");
}