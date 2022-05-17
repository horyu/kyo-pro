#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xxyy: [(Usize1, Usize1); m]
    };
    let mut vvv = vec![vec![false; n]; n];
    for i in 0..n {
        vvv[i][i] = true;
    }
    for (x, y) in xxyy {
        vvv[x][y] = true;
        vvv[y][x] = true;
    }
    for size in (1..=n).rev() {
        for ii in (0..n).combinations(size) {
            if ii.iter().all(|&i| ii.iter().all(|&j| vvv[i][j])) {
                println!("{size}");
                return;
            }
        }
    }
}
