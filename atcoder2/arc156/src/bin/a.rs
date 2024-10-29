#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        nnss: [(usize, Chars); n],
    };
    for (n, s) in nnss {
        let ttff = s.iter().copied().map(|c| c == '1').collect_vec();
        let mut ones = [0; 2];
        for (i, tf) in ttff.iter().copied().enumerate() {
            ones[i % 2] += usize::from(tf);
        }
        let sum = ones[0] + ones[1];
        if sum % 2 == 1 || (n == 3 && (ones[0] == 1 || ones[1] == 1)) {
            println!("-1");
            continue;
        }
        let mut rs = sum / 2;
        if sum == 2 {
            // 連続している場合
            if let Some(pos) = (0..(n - 1)).position(|i| ttff[i] && ttff[i + 1]) {
                if n == 4 {
                    // 0110 → 1100 → 1001 → 0000
                    rs += match pos {
                        0 | 2 => 1,
                        1 => 2,
                        _ => 0,
                    };
                } else {
                    rs += 1;
                }
            }
        }
        println!("{rs}");
    }
}
