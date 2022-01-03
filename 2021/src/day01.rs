// https://adventofcode.com/2021/day/1

pub fn part1(input: String) -> i32 {
    let values: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    if let Some((first, rest)) = values.split_first() {
        let mut count = 0;
        rest.iter().fold(*first, |acc, &x| {
            if acc < x {
                count += 1;
            }
            x
        });
        count
    } else {
        0
    }
}

pub fn part2(input: String) -> i32 {
    let values: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    if let Some((first, rest)) = values
        .as_slice()
        .windows(3)
        .collect::<Vec<_>>()
        .split_first()
    {
        let mut count = 0;
        rest.iter().fold(first.iter().sum::<u64>(), |acc, &x| {
            let current_sum = x.iter().sum();
            if acc < current_sum {
                count += 1;
            }
            current_sum
        });
        count
    } else {
        0
    }
}
