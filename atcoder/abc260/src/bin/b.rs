#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
 
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let mut rrss = vec![];
    let mut unused = vec![true; n];
    let mut ss = (0..n).collect_vec();
    ss.sort_by_key(|&s| 101 - aa[s]);
    for s in ss.into_iter().take(x) {
        rrss.push(s);
        unused[s] = false;
    }
    let mut uu = (0..n).collect_vec();
    uu.sort_by_key(|&t| 101 - bb[t]);
    let mut t_cnt = 0;
    for t in uu {
        if y <= t_cnt {
            break;
        }
        if unused[t] {
            unused[t] = false;
            rrss.push(t);
            t_cnt += 1;
        }
    }
    let mut uu = (0..n).collect_vec();
    uu.sort_by_key(|&u| 202 - aa[u] - bb[u]);
    let mut u_cnt = 0;
    for u in uu {
        if z <= u_cnt {
            break;
        }
        if unused[u] {
            unused[u] = false;
            rrss.push(u);
            u_cnt += 1;
        }
    }
    rrss.sort_unstable();
    println!("{}", rrss.into_iter().map(|rs| rs + 1).join("\n"));
}