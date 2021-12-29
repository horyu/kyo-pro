#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut ll: [isize; n]
    };
    if n < 3 {
        println!("0");
        return;
    }
    ll.sort_unstable();
    let mut count = 0;
    for i in 0..(n - 2) {
        let a = ll[i];
        for j in (i + 1)..(n - 1) {
            let b = ll[j];
            for k in (j + 1)..n {
                let c = ll[k];
                if (a == b) || (b == c) {
                    continue;
                }
                count += ((b - a < c) && (c < a + b)) as i32;
            }
        }
    }
    println!("{}", count);
}
