extern crate lazy_static;
extern crate regex;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("D1P1: {}", day1::puzzle1());
    println!("D1P2: {}", day1::puzzle2());
    println!("");
    println!("D2P1: {}", day2::puzzle1());
    println!("D2P2: {}", day2::puzzle2());
    println!("");
    println!("D3P1: {}", day3::puzzle1());
    println!("D3P2: {}", day3::puzzle2());
    println!("");
    println!("D4P1: {}", day4::puzzle1());
    println!("D4P2: {}", day4::puzzle2());
    println!("");
    println!("D5P1: {}", day5::puzzle1());
    println!("D5P2: {}", day5::puzzle2());
    println!("");
    println!("D6P1: {}", day6::puzzle1());
    println!("D6P2: {}", day6::puzzle2());
    println!("");
    println!("D7P1: {}", day7::puzzle1());
    println!("D7P2: {}", day7::puzzle2());
}
