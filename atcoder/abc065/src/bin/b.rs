#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n]
    };
    let mut v = vec![false; n];
    v[0] = true;
    let mut a = aa[0];
    let mut count = 1;
    loop {
        if a == 1 {
            println!("{}", count);
            return;
        }
        if v[a] {
            println!("-1");
            return;
        }
        v[a] = true;
        a = aa[a];
        count += 1;
    }
}
