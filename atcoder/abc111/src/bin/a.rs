#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: Chars
    };
    n.push('\n');
    n.iter().for_each(|c| {
        print!(
            "{}",
            match c {
                '1' => '9',
                '9' => '1',
                _ => *c,
            }
        )
    })
}
