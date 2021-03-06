pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;

pub fn noop(_inp: String) -> i32 {
    0
}

pub type DayFn = fn(String) -> i32;

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, noop),
        // 5 => (day05::part1, noop),
        // 6 => (day06::part1, noop),
        // 7 => (day07::part1, noop),
        // 8 => (day08::part1, noop),
        // 9 => (day09::part1, noop),
        // 10 => (day10::part1, noop),
        // 11 => (day11::part1, noop),
        // 12 => (day12::part1, noop),
        // 13 => (day13::part1, noop),
        // 14 => (day14::part1, noop),
        // 15 => (day15::part1, noop),
        // 16 => (day16::part1, noop),
        // 17 => (day17::part1, noop),
        // 18 => (day18::part1, noop),
        // 19 => (day19::part1, noop),
        // 20 => (day20::part1, noop),
        // 21 => (day21::part1, noop),
        // 22 => (day22::part1, noop),
        // 23 => (day23::part1, noop),
        // 24 => (day24::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
