#[macro_use]
extern crate clap;
extern crate lazy_static;
extern crate regex;

use clap::App;
use std::collections::HashMap;

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
mod day11;

fn build_function_map(
) -> HashMap<&'static str, Box<(Box<dyn Fn() -> usize>, Box<dyn Fn() -> usize>)>> {
    let mut function_map: HashMap<&str, Box<(Box<dyn Fn() -> usize>, Box<dyn Fn() -> usize>)>> =
        HashMap::new();
    function_map.insert(
        "1",
        Box::new((Box::new(day01::puzzle1), Box::new(day01::puzzle2))),
    );
    function_map.insert(
        "2",
        Box::new((Box::new(day02::puzzle1), Box::new(day02::puzzle2))),
    );
    function_map.insert(
        "3",
        Box::new((Box::new(day03::puzzle1), Box::new(day03::puzzle2))),
    );
    function_map.insert(
        "4",
        Box::new((Box::new(day04::puzzle1), Box::new(day04::puzzle2))),
    );
    function_map.insert(
        "5",
        Box::new((Box::new(day05::puzzle1), Box::new(day05::puzzle2))),
    );
    function_map.insert(
        "6",
        Box::new((Box::new(day06::puzzle1), Box::new(day06::puzzle2))),
    );
    function_map.insert(
        "7",
        Box::new((Box::new(day07::puzzle1), Box::new(day07::puzzle2))),
    );
    // function_map.insert(
    //     "8",
    //     Box::new((Box::new(day08::puzzle1, Box::new(day08::puzzle2))),
    // );
    function_map.insert(
        "9",
        Box::new((Box::new(day09::puzzle1), Box::new(day09::puzzle2))),
    );
    function_map.insert(
        "10",
        Box::new((Box::new(day10::puzzle1), Box::new(day10::puzzle2))),
    );
    function_map.insert(
        "11",
        Box::new((Box::new(day11::puzzle1), Box::new(day11::puzzle2))),
    );
    function_map
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.value_of("DAY").unwrap_or("0") {
        "0" => {
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
            println!("");
            println!("D10P1: {}", day11::puzzle1());
            println!("D10P2: {}", day11::puzzle2());
        }
        "8" => println!("P1: {}\nP2: {}", day08::puzzle1(), day08::puzzle2()),
        d => {
            let function_map = build_function_map();
            let res = function_map.get(d).unwrap();
            println!("P1: {}\nP2: {}", res.0(), res.1());
        }
    }
}
