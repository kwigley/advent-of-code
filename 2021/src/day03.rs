// https://adventofcode.com/2021/day/3

const WIDTH: usize = 12;
const COUNT: usize = 1000;

pub fn part1(input: String) -> i32 {
    let gamma = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
        .sum::<u32>();
    (gamma * (!gamma & ((1 << WIDTH) - 1))) as i32
}

pub fn part2(input: String) -> i32 {
    let nums = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            let mut x = 0;
            while x < oxy.len() {
                if (oxy[x] & 1 << i > 0) != one {
                    oxy.remove(x);
                } else {
                    x += 1;
                }
            }
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            let mut x = 0;
            while x < co2.len() {
                if (co2[x] & 1 << i > 0) == one {
                    co2.remove(x);
                } else {
                    x += 1;
                }
            }
            co2.first().copied()
        })
        .last()
        .unwrap();
    (oxy * co2) as i32
}
