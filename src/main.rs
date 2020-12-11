extern crate lazy_static;
extern crate regex;

mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    println!("D01P1: {}", day01::puzzle1());
    println!("D01P2: {}", day01::puzzle2());
    println!("");
    println!("D02P1: {}", day02::puzzle1());
    println!("D02P2: {}", day02::puzzle2());
    println!("");
    println!("D03P1: {}", day03::puzzle1());
    println!("D03P2: {}", day03::puzzle2());
    println!("");
    println!("D04P1: {}", day04::puzzle1());
    println!("D04P2: {}", day04::puzzle2());
    println!("");
    println!("D05P1: {}", day05::puzzle1());
    println!("D05P2: {}", day05::puzzle2());
    println!("");
    println!("D06P1: {}", day06::puzzle1());
    println!("D06P2: {}", day06::puzzle2());
    println!("");
    println!("D07P1: {}", day07::puzzle1());
    println!("D07P2: {}", day07::puzzle2());
    println!("");
    println!("D08P1: {}", day08::puzzle1());
    println!("D08P2: {}", day08::puzzle2());
    println!("");
    println!("D09P1: {}", day09::puzzle1());
    println!("D09P2: {}", day09::puzzle2());
    println!("");
    println!("D10P1: {}", day10::puzzle1());
    println!("D10P2: {}", day10::puzzle2());
}
