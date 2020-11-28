#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abab: [(usize, usize); n]
    };
    abab.sort_unstable_by_key(|ab| ab.0);
    let mut sum = 0;
    let mut count = 0;
    while count < m {
        if abab[0].1 == 0 {
            abab.remove(0);
        }
        if count + abab[0].1 <= m {
            sum += abab[0].0 * abab[0].1;
            count += abab[0].1;
            abab[0].1 = 0;
        } else {
            sum += abab[0].0;
            count += 1;
            abab[0].1 -= 1;
        }
    }
    println!("{}", sum);
}
