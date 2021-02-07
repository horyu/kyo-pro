#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let rs = s.windows(3).filter(|arr| arr == &['A', 'B', 'C']).count();
    println!("{}", rs);
}
