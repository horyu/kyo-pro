#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        llrr: [(Usize1, Usize1); q],
    };
    let mut sum1 = vec![0; n + 1];
    let mut sum2 = vec![0; n + 1];
    let mut sums = vec![0; n + 1];
    let mut vv1 = vec![];
    let mut vv2 = vec![];
    let mut vvs = vec![];
    for (i, c) in s.iter().copied().enumerate() {
        sum1[i + 1] = sum1[i] + i32::from(c == '1');
        sum2[i + 1] = sum2[i] + i32::from(c == '2');
        sums[i + 1] = sums[i] + i32::from(c == '/');
        let vv = match c {
            '1' => &mut vv1,
            '2' => &mut vv2,
            _ => &mut vvs,
        };
        vv.push(i);
    }
    // eprintln!(" {}", s.iter().join(""));
    // eprintln!("{}", sum1.iter().copied().join(""));
    // eprintln!("{}", sum2.iter().copied().join(""));
    for (l, r) in llrr {
        if sums[r + 1] == sums[l] {
            println!("0");
            continue;
        }
        // 二分探索
        let max1 = sum1[r + 1] - sum1[l];
        let max2 = sum2[r + 1] - sum2[l];
        let max = max1.min(max2);
        let mut ok = 0;
        let mut ng = max + 1;
        while 1 < ng - ok {
            let mid = (ok + ng) / 2;
            let mut tf = false;
            if let Some(&pos1_mid) = vv1.get(vv1.partition_point(|&v1| v1 < l) + mid as usize - 1) {
                if let Some(&poss) = vvs.get(vvs.partition_point(|&vs| vs < pos1_mid)) {
                    if poss < r && mid <= sum2[r + 1] - sum2[poss] {
                        tf = true;
                    }
                }
            }
            if tf {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let rs = 1 + 2 * ok;
        println!("{rs}");
    }
}
