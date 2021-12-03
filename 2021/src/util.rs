use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Helper from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_ints(path: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let lines = read_lines(path).unwrap();

    for line in lines {
        let num = line.unwrap().parse::<i32>().unwrap();
        out.push(num);
    }

    out
}

// #[macro_export]
// macro_rules! set(
//     { $($key:expr),+ } => {
//         {
//             let mut m = ::std::collections::HashSet::new();
//             $(
//                 m.insert($key);
//             )+
//             m
//         }
//      };
// );

// #[macro_export]
// macro_rules! map(
//     { $($key:expr => $val:expr),+ } => {
//         {
//             let mut m = ::std::collections::HashMap::new();
//             $(
//                 m.insert($key, $val);
//             )+
//             m
//         }
//      };
// );
