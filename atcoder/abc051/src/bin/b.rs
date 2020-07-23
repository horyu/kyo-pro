#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        k: isize,
        s: isize
    };
    let upper = std::cmp::min(k, s);
    let mut count = 0;
    // 0 <= x <= y <= z <= k で考える
    for x in 0..=upper {
        for y in x..=upper {
            let z = s - x - y;
            if z > k {
                continue;
            }
            if (z < 0) || (z < y) {
                break;
            }
            count += if (x == y) && (y == z) {
                1
            } else if (x == y) || (y == z) {
                3
            } else {
                6
            };
        }
    }
    println!("{}", count);
}
