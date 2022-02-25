#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aabb: [(Usize1, Usize1); m]
    };
    aabb.sort_unstable();
    aabb.dedup();
    let mut dd = vec![0; n];
    let mut ppp = vec![vec![]; n];
    for (a, b) in aabb {
        ppp[a].push(b);
        dd[b] += 1;
    }
    let mut bh = BinaryHeap::new();
    for (i, &d) in dd.iter().enumerate() {
        if d == 0 {
            bh.push(std::cmp::Reverse(i));
        }
    }

    let mut rs = vec![];
    while let Some(std::cmp::Reverse(q)) = bh.pop() {
        rs.push(q);
        for &p in &ppp[q] {
            dd[p] -= 1;
            if dd[p] == 0 {
                bh.push(std::cmp::Reverse(p));
            }
        }
    }
    if rs.len() == n {
        println!("{}", rs.into_iter().map(|x| x + 1).join(" "));
    } else {
        println!("-1");
    }
}
