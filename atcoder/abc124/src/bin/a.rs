#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut ab: [usize; 2],
    };
    ab.sort();
    println!("{}", ab[1] + if ab[0] == ab[1] { ab[0] } else { ab[1] - 1 });
}
