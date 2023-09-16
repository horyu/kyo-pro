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
        m: usize,
        // s1: Chars,
        // s2: Chars,
        // s3: Chars,
        ss: [Chars; 3],
    };
    let mut rs = !0;
    for target in '0'..='9' {
        let iii = ss
            .iter()
            .map(|s| s.iter().positions(|&c| c == target).collect_vec())
            .collect_vec();
        if iii.iter().any(|ii| ii.is_empty()) {
            continue;
        }
        for oo in (0..3).permutations(3) {
            let mut tmp = iii[oo[0]][0];
            let mut cnt = 0;
            if let Some(&i) = iii[oo[1]].iter().find(|&&i| tmp < i) {
                tmp = i;
            } else {
                tmp = iii[oo[1]][0];
                cnt += 1;
            }
            if let Some(&i) = iii[oo[2]].iter().find(|&&i| tmp < i) {
                tmp = i;
            } else {
                tmp = iii[oo[2]][0];
                cnt += 1;
            }
            rs = rs.min(cnt * m + tmp);
        }
    }
    if rs == !0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
