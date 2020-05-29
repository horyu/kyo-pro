#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut n: isize,
        ngs: [isize; 3]
    };
    if ngs.contains(&n) {
        println!("NO");
        return;
    }
    let mut count = 100;
    'outer: while count > 0 {
        for &diff in [3, 2, 1].iter() {
            let next = n - diff;
            if next <= 0 {
                println!("YES");
                return;
            }
            if !ngs.contains(&next) {
                n = next;
                count -= 1;
                continue 'outer;
            }
        }
        println!("NO");
        return;
    }
    println!("NO");
}
