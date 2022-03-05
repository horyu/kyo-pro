#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        q: usize,
        llrr: [(usize, usize); q]
    };
    // Mo's algorithm
    // https://ei1333.hateblo.jp/entry/2017/09/11/211011

    let size = n.sqrt();
    let mut qqq = vec![vec![]; n / size + 5];
    for (i, &(l, r)) in llrr.iter().enumerate() {
        let l = l - 1;
        qqq[l / size].push((r, l, i));
    }

    let mut rrss = vec![0; q];
    for mut qq in qqq {
        if qq.is_empty() {
            continue;
        }
        qq.sort_unstable();
        let mut vv = vec![0; n];
        let mut crr = 0;
        let mut l = qq[0].1;
        let mut r = l;
        for (qr, ql, qi) in qq {
            while qr > r {
                crr += vv[aa[r]] % 2;
                vv[aa[r]] += 1;
                r += 1;
            }
            while ql > l {
                vv[aa[l]] -= 1;
                crr -= vv[aa[l]] % 2;
                l += 1;
            }
            while ql < l {
                l -= 1;
                crr += vv[aa[l]] % 2;
                vv[aa[l]] += 1;
            }
            rrss[qi] = crr;
        }
    }
    println!("{}", rrss.into_iter().join("\n"));
}
