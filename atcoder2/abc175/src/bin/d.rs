#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        pp: [Usize1; n],
        cc: [isize; n],
    };
    let mut rs = isize::MIN;
    for mut x in 0..n {
        let mut x_to_i = vec![std::usize::MAX; n];
        x_to_i[x] = 0;
        let mut ss = vec![0isize];
        for r in 1..=k {
            let new_s = ss.last().unwrap() + cc[x];
            ss.push(new_s);
            rs = rs.max(new_s);
            x = pp[x];
            if x_to_i[x] != std::usize::MAX {
                // ループ
                let l = x_to_i[x];
                let loop_size = r - l;
                let loop_num = ((k - l) / loop_size) as isize;
                let mut tmp = ss[l];
                tmp += (ss[r] - ss[l]) * loop_num;
                tmp += ss[l..=(l + (k - l) % loop_size)].iter().max().unwrap() - ss[l];
                rs = rs.max(tmp);
                if 1 < loop_num {
                    let mut tmp = ss[l];
                    tmp += (ss[r] - ss[l]) * (loop_num - 1);
                    tmp += ss[l..(l + loop_size)].iter().max().unwrap() - ss[l];
                    rs = rs.max(tmp);
                }
                break;
            }
            x_to_i[x] = r;
        }
    }
    println!("{rs}");
}
