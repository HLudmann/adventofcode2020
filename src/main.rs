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
    println!("{}", day1::puzzle1());
    println!("{}", day1::puzzle2());
    println!("");
    println!("{}", day2::puzzle1());
    println!("{}", day2::puzzle2());
    println!("");
    println!("{}", day3::puzzle1());
    println!("{}", day3::puzzle2());
    println!("");
    println!("{}", day4::puzzle1());
    println!("{}", day4::puzzle2());
    println!("");
    println!("{}", day5::puzzle1());
    println!("{}", day5::puzzle2());
    println!("");
    println!("{}", day6::puzzle1());
    println!("{}", day6::puzzle2());
    println!("");
    println!("{}", day7::puzzle1());
    println!("{}", day7::puzzle2());
}
