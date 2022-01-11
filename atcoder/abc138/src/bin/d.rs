#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        aabb: [(usize, usize); n - 1],
        ppxx: [(usize, usize); q],
    };
    let mut vvv = vec![vec![]; n + 1];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    let mut cnts = vec![0; n + 1];
    for (p, x) in ppxx {
        cnts[p] += x;
    }

    let mut rs = vec![0; n + 1];
    dfs(&mut rs, &vvv, &cnts, 1, 0);

    println!("{}", rs[1..].iter().join(" "));
}

#[allow(clippy::ptr_arg)]
fn dfs(rs: &mut Vec<usize>, vvv: &Vec<Vec<usize>>, cnts: &Vec<usize>, crr: usize, pre: usize) {
    rs[crr] = rs[pre] + cnts[crr];
    for &next in &vvv[crr] {
        if next != pre {
            dfs(rs, vvv, cnts, next, crr);
        }
    }
}
