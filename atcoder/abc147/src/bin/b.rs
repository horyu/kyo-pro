#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut rs = 0;
    let len = s.len();
    for (a, b) in s[..(len / 2)]
        .iter()
        .zip(s[(len / 2 + (len % 2 != 0) as usize)..].iter().rev())
    {
        rs += (a != b) as usize;
    }
    println!("{}", rs);
}
