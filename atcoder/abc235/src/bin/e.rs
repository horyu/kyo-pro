#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut aabbcc: [(Usize1, Usize1, usize); m],
        uuvvww: [(Usize1, Usize1, usize); q],
    };
    // https://dai1741.github.io/maximum-algo-2012/docs/minimum-spanning-tree/
    aabbcc.sort_unstable_by_key(|abc| abc.2);

    let iiuuvvww = uuvvww
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_i, uvw)| uvw.2)
        .collect_vec();
    let mut rs = vec![false; q];
    let mut uf = UnionFind::new(n);
    let mut j = 0;
    for &(a, b, c) in &aabbcc {
        while let Some(&(i, (u, v, w))) = iiuuvvww.get(j) {
            if w > c {
                break;
            }
            let tf = !uf.equiv(u, v);
            rs[i] = tf;
            j += 1;
        }
        uf.union(a, b);
    }
    println!(
        "{}",
        rs.into_iter()
            .map(|tf| ["No", "Yes"][tf as usize])
            .join("\n")
    );
}
