#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    };
    let mut uf = petgraph::unionfind::UnionFind::new(h * w);
    let to_k = |r: usize, c: usize| -> usize { r * w + c };
    let mut vvv = vec![vec![false; w]; h];
    for _ in 0..q {
        input! {p: u8};
        if p == 1 {
            input! {r: Usize1, c: Usize1};
            if !vvv[r][c] {
                vvv[r][c] = true;
                if r > 0 && vvv[r - 1][c] {
                    uf.union(to_k(r, c), to_k(r - 1, c));
                }
                if r < h - 1 && vvv[r + 1][c] {
                    uf.union(to_k(r, c), to_k(r + 1, c));
                }
                if c > 0 && vvv[r][c - 1] {
                    uf.union(to_k(r, c), to_k(r, c - 1));
                }
                if c < w - 1 && vvv[r][c + 1] {
                    uf.union(to_k(r, c), to_k(r, c + 1));
                }
            }
        } else {
            input! {ri: Usize1, ci: Usize1, rj: Usize1, cj: Usize1};
            let tf = vvv[ri][ci] && vvv[rj][cj] && uf.equiv(to_k(ri, ci), to_k(rj, cj));
            println!("{}", ["No", "Yes"][tf as usize]);
        }
    }
}
