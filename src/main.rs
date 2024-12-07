#[path = "day01/day01.rs"]
mod day01;
#[path = "day02/day02.rs"]
mod day02;
#[path = "day03/day03.rs"]
mod day03;
#[path = "day04/day04.rs"]
mod day04;
#[path = "day05/day05.rs"]
mod day05;
#[path = "day06/day06.rs"]
mod day06;
#[path = "day07/day07.rs"]
mod day07;

fn main() {
    println!("--- Day1 ---");
    day01::day_one(r"src\day01\day01_input.txt").unwrap();
    println!("--- Day 2 ---");
    day02::day_two(r"src\day02\day02_input.txt").unwrap();
    println!("--- Day 3 ---");
    day03::day_three(r"src\day03\day03_input.txt");
    println!("--- Day 4 ---");
    day04::day_four(r"src\day04\day04_input.txt");
    println!("--- Day 5 ---");
    day05::day_five(r"src\day05\day05_input.txt");
    println!("--- Day 6 ---");
    day06::day_six(r"src\day06\day06_input.txt").unwrap();
    println!("--- Day 7 ---");
    day07::day_seven(r"src\day07\day07_input.txt");
}