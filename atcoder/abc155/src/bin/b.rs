#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    for a in aa {
        if (a % 2 == 0) && !((a % 3 == 0) || (a % 5 == 0)) {
            println!("DENIED");
            return;
        }
    }
    println!("APPROVED");
}
