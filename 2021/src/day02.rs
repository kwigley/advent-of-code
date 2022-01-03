// https://adventofcode.com/2021/day/2

pub fn part1(input: String) -> i32 {
    let (f, d) =
        input
            .lines()
            .map(|l| l.split_once(" ").unwrap())
            .fold((0, 0), |(f, d), (k, v)| {
                match (k, v.parse::<i32>().unwrap()) {
                    ("forward", v) => (f + v, d),
                    ("down", v) => (f, d + v),
                    ("up", v) => (f, d - v),
                    _ => unreachable!(),
                }
            });
    f * d
}

pub fn part2(input: String) -> i32 {
    let (f, d, _) = input.lines().map(|l| l.split_once(" ").unwrap()).fold(
        (0, 0, 0),
        |(f, d, a), (k, v)| match (k, v.parse::<i32>().unwrap()) {
            ("forward", v) => (f + v, d + a * v, a),
            ("down", v) => (f, d, a + v),
            ("up", v) => (f, d, a - v),
            _ => unreachable!(),
        },
    );
    f * d
}
