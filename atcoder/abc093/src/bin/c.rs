#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    abc.sort_unstable();
    let diff = (abc[2] - abc[1]) + (abc[2] - abc[0]);
    let rs = if diff % 2 == 0 {
        diff / 2
    } else {
        diff / 2 + 2
    };
    println!("{}", rs);
}
