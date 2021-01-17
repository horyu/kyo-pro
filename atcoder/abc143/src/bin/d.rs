#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ll: [usize; n]
    };
    ll.sort_unstable();
    let mut count = 0;
    for l in 0..(n - 2) {
        for c in (l + 1)..(n - 1) {
            let rr = ll[(c + 1)..].binary_search(&(ll[l] + ll[c] - 1));
            count += match rr {
                Ok(r) => r + 1,
                Err(r) => r,
            };
        }
    }
    println!("{}", count);
}
