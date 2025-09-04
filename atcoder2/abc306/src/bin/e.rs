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
        k: usize,
        q: usize,
        xxyy: [(Usize1, usize); q],
    };
    // f(A) = b1 + ... + bk
    let mut rs = 0;
    let mut aa = vec![0; n];
    let mut main = BTreeSet::new();
    let mut yobi = BTreeSet::new();
    for (x, y) in xxyy {
        let pre_a = aa[x];
        if main.remove(&(pre_a, x)) {
            rs -= pre_a;
        } else {
            yobi.remove(&(pre_a, x));
        }

        if main.len() < k {
            if let Some((yobi_max, yobi_max_idx)) = yobi.pop_last() {
                rs += yobi_max;
                main.insert((yobi_max, yobi_max_idx));
            }
        }

        if main.len() == k {
            let (main_min, main_min_idx) = main.iter().min().copied().unwrap_or_default();
            if main_min < y {
                rs += y - main_min;
                main.insert((y, x));

                main.remove(&(main_min, main_min_idx));
                yobi.insert((main_min, main_min_idx));
            } else {
                yobi.insert((y, x));
            }
        } else {
            rs += y;
            main.insert((y, x));
        }

        aa[x] = y;
        println!("{rs}");
        // eprintln!("{x} {y}: {aa:?}");
        // eprintln!("{}", main.iter().map(|(k, v)| format!("({k},{v})")).join(","));
        // eprintln!("{}", yobi.iter().map(|(k, v)| format!("({k},{v})")).join(","));
    }
}
