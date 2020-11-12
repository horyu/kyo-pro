#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: usize
    };
    let mut hs = std::collections::HashSet::new();
    let mut a = s;
    hs.insert(a);
    let m = (2usize..)
        .find(|_i| {
            a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
            if hs.contains(&a) {
                true
            } else {
                hs.insert(a);
                false
            }
        })
        .unwrap();
    println!("{}", m);
}
