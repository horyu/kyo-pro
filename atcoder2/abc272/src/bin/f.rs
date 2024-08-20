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
        s: Chars,
        t: Chars,
    };
    // https://atcoder.jp/contests/abc272/editorial/4980
    // https://atcoder.jp/contests/abc272/submissions/35720986
    let x = chain!(&s, &s, &['!'], &t, &t, &['~'],)
        .copied()
        .collect_vec();
    let sa = ac_library::suffix_array_arbitrary(&x);

    let mut tmp = 0usize;
    let mut rs = 0usize;
    for i in sa {
        if i < n {
            tmp += 1;
        }
        // 1つ目のtの範囲内であれば、それ以前に出現したsのsuffixのインデックスの数が対象
        if ((2 * n + 1)..=(3 * n)).contains(&i) {
            rs += tmp;
        }
    }

    println!("{rs}");
}
