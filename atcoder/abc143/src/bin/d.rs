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
            for r in (c + 1)..n {
                if ll[r] < ll[l] + ll[c] {
                    count += 1;
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", count);
}
