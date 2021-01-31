#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
        k: usize,
        ccdd: [[Usize1; 2]; k],
    };
    if k == 1 {
        println!("0");
        return;
    }
    let mut vv = vec![vec![0; n]; n];
    for (a, b) in aabb {
        vv[a][b] += 1;
        vv[b][a] += 1;
    }
    let mut max = 0;
    for mut v in ccdd.into_iter().multi_cartesian_product() {
        v.sort();
        v.dedup();
        let mut tmp = 0;
        for (c, d) in v.into_iter().tuple_combinations() {
            tmp += vv[c][d];
        }
        max = max.max(tmp);
    }
    println!("{}", max);
}
