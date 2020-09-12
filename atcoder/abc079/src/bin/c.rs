#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        abcd: Chars
    };
    let a = abcd[0].to_digit(10).unwrap() as i8;
    let b = abcd[1].to_digit(10).unwrap() as i8;
    let c = abcd[2].to_digit(10).unwrap() as i8;
    let d = abcd[3].to_digit(10).unwrap() as i8;
    let bb = vec![b, -b];
    let cc = vec![c, -c];
    let dd = vec![d, -d];
    for ((b, c), d) in bb.iter().cartesian_product(cc).cartesian_product(dd) {
        if a + b + c + d == 7 {
            println!("{}{:+}{:+}{:+}=7", a, b, c, d);
            return;
        }
    }
}
