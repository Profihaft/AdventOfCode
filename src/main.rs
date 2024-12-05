mod day01 {
    pub mod day01;
}
mod day02 {
    pub mod day02;
}
mod day03{
    pub mod day03;
}
mod day04{
    pub mod day04;
}
mod day05 {
    pub mod day05;
}

use day01::day01::*;
use day02::day02::*;
use day03::day03::*;
use day04::day04::*;
use day05::day05::*;

fn main() {
    println!("--- Day1 ---");
    day_one(r"src\day01\day01_input.txt").unwrap();
    println!("--- Day 2 ---");
    day_two(r"src\day02\day02_input.txt").unwrap();
    println!("--- Day 3 ---");
    day_three(r"src\day03\day03_input.txt");
    println!("--- Day 4 ---");
    day_four(r"src\day04\day04_input.txt");
    println!("--- Day 5 ---");
    day_five(r"src\day05\day05_input.txt");
}