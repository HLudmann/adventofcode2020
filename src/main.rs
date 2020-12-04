extern crate lazy_static;
extern crate regex;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

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
}