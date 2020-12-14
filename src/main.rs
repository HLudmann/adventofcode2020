#[macro_use]
extern crate clap;
extern crate lazy_static;
extern crate regex;

use clap::App;

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
mod day12;
mod day13;
mod day14;

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
            println!("D11P1: {}", day11::puzzle1());
            println!("D11P2: {}", day11::puzzle2());
            println!("");
            println!("D12P1: {}", day12::puzzle1());
            println!("D12P2: {}", day12::puzzle2());
            println!("");
            println!("D13P1: {}", day13::puzzle1());
            println!("D13P2: {}", day13::puzzle2());
            println!("");
            println!("D14P1: {}", day14::puzzle1());
            println!("D14P2: {}", day14::puzzle2());
        }
        "1" => println!("P1: {}\nP2: {}", day01::puzzle1(), day01::puzzle2()),
        "2" => println!("P1: {}\nP2: {}", day02::puzzle1(), day02::puzzle2()),
        "3" => println!("P1: {}\nP2: {}", day03::puzzle1(), day03::puzzle2()),
        "4" => println!("P1: {}\nP2: {}", day04::puzzle1(), day04::puzzle2()),
        "5" => println!("P1: {}\nP2: {}", day05::puzzle1(), day05::puzzle2()),
        "6" => println!("P1: {}\nP2: {}", day06::puzzle1(), day06::puzzle2()),
        "7" => println!("P1: {}\nP2: {}", day07::puzzle1(), day07::puzzle2()),
        "8" => println!("P1: {}\nP2: {}", day08::puzzle1(), day08::puzzle2()),
        "9" => println!("P1: {}\nP2: {}", day09::puzzle1(), day09::puzzle2()),
        "10" => println!("P1: {}\nP2: {}", day10::puzzle1(), day10::puzzle2()),
        "11" => println!("P1: {}\nP2: {}", day11::puzzle1(), day11::puzzle2()),
        "12" => println!("P1: {}\nP2: {}", day12::puzzle1(), day12::puzzle2()),
        "13" => println!("P1: {}\nP2: {}", day13::puzzle1(), day13::puzzle2()),
        "14" => println!("P1: {}\nP2: {}", day14::puzzle1(), day14::puzzle2()),
        d => println!("day not recognized: '{}'", d),
    }
}
