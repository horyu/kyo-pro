#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut count = 0;
    let mut set = std::collections::HashSet::new();
    for a in aa {
        if set.contains(&a) {
            count += 1;
        } else {
            set.insert(a);
        }
    }
    println!("{}", count);
}
