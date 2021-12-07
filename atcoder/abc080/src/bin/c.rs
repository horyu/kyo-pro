#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        fff: [[usize; 10]; n],
        ppp: [[isize; 11]; n],
    };
    let mut max = std::isize::MIN;
    for ttff in (0..10)
        .map(|_| [0usize, 1usize].iter())
        .multi_cartesian_product()
        .skip(1)
    {
        let mut sum = 0;
        for (ff, pp) in fff.iter().zip(ppp.iter()) {
            sum += pp[ff
                .iter()
                .zip(ttff.iter())
                .filter(|(x, y)| **x == 1 && ***y == 1)
                .count()];
        }
        max = max.max(sum);
    }
    println!("{}", max);
}
