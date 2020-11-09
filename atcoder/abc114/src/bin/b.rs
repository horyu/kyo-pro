#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let rs = s
        .windows(3)
        .map(|arr| {
            let num =
                (arr[0] as isize - 48) * 100 + (arr[1] as isize - 48) * 10 + (arr[2] as isize - 48);
            (753 - num).abs()
        })
        .min()
        .unwrap();
    println!("{}", rs);
}
