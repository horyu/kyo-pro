#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut max = std::isize::MIN;
    for i in 0..n {
        let mut t_max = std::isize::MIN;
        let mut a_max = std::isize::MIN;
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut t_score = 0;
            let mut a_score = 0;
            let (l, r) = if i < j { (i, j) } else { (j, i) };
            for (k, a) in aa[l..=r].iter().enumerate() {
                if k % 2 == 0 {
                    t_score += a;
                } else {
                    a_score += a;
                }
            }
            if a_score > a_max {
                t_max = t_score;
                a_max = a_score;
            }
        }
        max = max.max(t_max);
    }
    println!("{}", max);
}
