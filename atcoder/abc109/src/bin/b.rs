#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ww: [String; n]
    };
    let mut hs = std::collections::HashSet::new();
    for tmp in ww.windows(2) {
        let a = &tmp[0];
        let b = &tmp[1];
        hs.insert(a);
        if (a.chars().last().unwrap() != b.chars().next().unwrap()) || hs.contains(&b) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
