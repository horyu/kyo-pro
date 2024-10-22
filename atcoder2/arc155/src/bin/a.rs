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
        t: usize,
        nnkkss: [(usize, usize, Chars); t],
    };
    for (n, k, s) in nnkkss {
        // Ss と sS が回分になる長さkの文字列 S を考える
        // sの逆順をtとすると、 S = t[st]* の形になる
        let f = |i: usize| -> char {
            if (i / n) % 2 == 0 {
                s[n - 1 - (i % n)]
            } else {
                s[i % n]
            }
        };
        let c = (n + k).div_ceil(2);
        let mut ok = true;
        // sに注目して sS が回分か
        for i in 0..=(c.min(n - 1)) {
            if i < k {
                ok &= s[i] == f(k - 1 - i);
            } else {
                ok &= s[i] == s[n + k - i - 1];
            }
        }
        // sに注目して Ss が回分か
        for i in 0..=(c.min(n - 1)) {
            if i < k {
                ok &= s[n - 1 - i] == f(n + k - 1 - i);
            } else {
                ok &= s[n - 1 - i] == s[i - k];
            }
        }
        let rs = ["No", "Yes"][ok as usize];
        println!("{rs}");
    }
}
