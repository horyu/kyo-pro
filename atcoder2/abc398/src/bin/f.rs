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
        s: Chars,
    };
    // https://snuke.hatenablog.com/entry/2014/12/02/235837
    // 回文 Manacher
    let s = itertools::Itertools::intersperse(s.into_iter(), '#').collect_vec();
    let n = s.len();
    let mut rr = vec![0; n];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while j <= i && i + j < n && s[i - j] == s[i + j] {
            j += 1;
        }
        rr[i] = j;
        let mut k = 1;
        while k <= i && k + rr[i - k] < j {
            rr[i + k] = rr[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    // eprintln!("{rr:?}");
    for c in (n / 2)..n {
        let r = rr[c];
        if c + r < n {
            continue;
        }
        // println!("{c} {r}");
        let rs = chain!(s.iter(), s[..(c + 1 - r)].iter().rev())
            .filter(|&&c| c != '#')
            .join("");
        println!("{rs}");
        return;
    }
}
