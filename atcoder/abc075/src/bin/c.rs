#![allow(unused_imports)]
// use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    // https://blog.hamayanhamayan.com/entry/2017/10/15/002234
    let mut rs = 0;
    for i in 0..m {
        let mut uf = UnionFind::new(n);
        #[allow(clippy::needless_range_loop)]
        for j in 0..m {
            if i != j {
                let (a, b) = aabb[j];
                uf.union(a, b);
            }
        }
        if (0..n).filter(|&j| j == uf.find(j)).count() > 1 {
            rs += 1;
        }
    }
    println!("{}", rs);
}
