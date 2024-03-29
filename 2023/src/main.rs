use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn run_day_main(day: u8, part2: bool, input: String) {
    match day {
        1 => day1::solution(input, part2),
        2 => day2::solution(input, part2),
        3 => day3::solution(input, part2),
        4 => day4::solution(input, part2),
        5 => day5::solution(input, part2),
        6 => day6::solution(input, part2),
        7 => day7::solution(input, part2),
        8 => day8::solution(input, part2),
        9 => day9::solution(input, part2),
        10 => day10::solution(input, part2),
        11 => day11::solution(input, part2),
        12 => day12::solution(input, part2),
        13 => day13::solution(input, part2),
        14 => day14::solution(input, part2),
        15 => day15::solution(input, part2),
        16 => day16::solution(input, part2),
        17 => day17::solution(input, part2),
        18 => day18::solution(input, part2),
        19 => day19::solution(input, part2),
        20 => day20::solution(input, part2),
        21 => day21::solution(input, part2),
        22 => day22::solution(input, part2),
        23 => day23::solution(input, part2),
        24 => day24::solution(input, part2),
        25 => day25::solution(input, part2),
        _ => eprintln!("Invalid day!"),
    }
}

fn run_solution(day: u8, part2: bool) {
    let input = fs::read_to_string(format!("./inputs/{}.txt", day)).expect("Unable to read from file.");
    run_day_main(day, part2, input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<u8>().expect("Invalid day.");
    let part2 = &args[2].parse::<u8>().expect("Invalid part2 flag.");

    run_solution(*day, *part2 == 2 as u8);
}
