#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut set = std::collections::HashSet::new();
    for a in aa {
        let mut up = a;
        while up < 1_000_000_000 {
            up *= 2;
        }
        set.insert(up);
    }
    println!("{}", set.len());
}
