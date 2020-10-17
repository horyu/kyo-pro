#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    use std::collections::HashSet;
    let mut max = 0;
    for i in 1..=(n - 1) {
        let mae: HashSet<_> = (&s[0..i]).iter().collect();
        let ato: HashSet<_> = (&s[i..]).iter().collect();
        let num = mae.intersection(&ato).count();
        max = std::cmp::max(max, num);
    }
    println!("{}", max);
}
