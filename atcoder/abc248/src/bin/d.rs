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
        llrrxx: [(usize, usize, Usize1); q]
    };
    let mut vvv = vec![vec![]; n];
    for (i, a) in aa.into_iter().enumerate() {
        vvv[a].push(i);
    }
    for (l, r, x) in llrrxx {
        let ri = vvv[x].partition_point(|&v| v < r);
        let li = vvv[x].partition_point(|&v| v < l - 1);
        let rs = ri - li;
        println!("{rs}");
    }
    // Mo's algorithm
    /*
    let size = 200000.sqrt();
    let mut qqq = vec![vec![]; size + 2];
    for (i, (l, r, x)) in llrrxx.into_iter().enumerate() {
        let l = l - 1;
        qqq[l / size].push((r, l, x, i));
    }
    let mut rrss = vec![0; q];
    for mut qq in qqq {
        if qq.is_empty() {
            continue;
        }
        qq.sort_unstable();
        let mut l = qq[0].1;
        let mut r = l;
        let mut vv = vec![0; n];
        for (qr, ql, qx, qi) in qq {
            while qr > r {
                vv[aa[r]] += 1;
                r += 1;
            }
            while ql > l {
                vv[aa[l]] -= 1;
                l += 1;
            }
            while ql < l {
                l -= 1;
                vv[aa[l]] += 1;
            }
            rrss[qi] = vv[qx];
        }
    }
    for rs in rrss {
        println!("{rs}");
    }
     */
}
