extern crate lazy_static;
extern crate regex;

mod common;
mod day1;
mod day2;

fn main() {
    println!("{}", day1::puzzle1());
    println!("{}", day1::puzzle2());
    println!("");
    println!("{}", day2::puzzle1());
    println!("{}", day2::puzzle2());
}
