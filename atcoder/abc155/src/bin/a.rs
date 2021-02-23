#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3],
    };
    abc.sort_unstable();
    abc.dedup();
    println!("{}", ["No", "Yes"][(abc.len() == 2) as usize]);
}
