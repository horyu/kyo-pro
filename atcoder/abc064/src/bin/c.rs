#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    let mut min_colors = 0;
    let mut free_colors = 0;
    let mut pre = std::usize::MAX;
    for a in aa {
        let num = a / 400;
        if num >= 3200 / 400 {
            free_colors += 1;
        } else if pre != num {
            pre = num;
            min_colors += 1;
        }
    }
    if (min_colors == 0) && (free_colors > 0) {
        min_colors += 1;
        free_colors -= 1;
    }
    println!("{} {}", min_colors, min_colors + free_colors);
}
