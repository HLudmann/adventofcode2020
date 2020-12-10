extern crate lazy_static;
extern crate regex;

mod common;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("D01P1: {}", day1::puzzle1());
    println!("D01P2: {}", day1::puzzle2());
    println!("");
    println!("D02P1: {}", day2::puzzle1());
    println!("D02P2: {}", day2::puzzle2());
    println!("");
    println!("D03P1: {}", day3::puzzle1());
    println!("D03P2: {}", day3::puzzle2());
    println!("");
    println!("D04P1: {}", day4::puzzle1());
    println!("D04P2: {}", day4::puzzle2());
    println!("");
    println!("D05P1: {}", day5::puzzle1());
    println!("D05P2: {}", day5::puzzle2());
    println!("");
    println!("D06P1: {}", day6::puzzle1());
    println!("D06P2: {}", day6::puzzle2());
    println!("");
    println!("D07P1: {}", day7::puzzle1());
    println!("D07P2: {}", day7::puzzle2());
    println!("");
    println!("D08P1: {}", day8::puzzle1());
    println!("D08P2: {}", day8::puzzle2());
    println!("");
    println!("D09P1: {}", day9::puzzle1());
    println!("D09P2: {}", day9::puzzle2());
    println!("");
    println!("D10P1: {}", day10::puzzle1());
    println!("D10P2: {}", day10::puzzle2());
}
