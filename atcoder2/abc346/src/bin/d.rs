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
        aa: [usize; n],
    };
    // 左から (0101.., 1010..) のコストを累積和で求める
    let mut ll = vec![[0; 2]; n + 1];
    for (i, (a, c)) in izip!(aa.iter().copied(), s.iter().copied()).enumerate() {
        ll[i + 1] = ll[i];
        if (i % 2 == 0) ^ (c == '0') {
            ll[i + 1][0] += a;
        } else {
            ll[i + 1][1] += a;
        }
    }
    // 右から (0101.., 1010..) のコストを累積和で求める
    let mut rr = vec![[0; 2]; n + 1];
    for (i, (a, c)) in izip!(aa.iter().copied(), s.iter().copied())
        .enumerate()
        .rev()
    {
        rr[i] = rr[i + 1];
        if ((n - 1 - i) % 2 == 0) ^ (c == '0') {
            rr[i][0] += a;
        } else {
            rr[i][1] += a;
        }
    }
    // for (i, l) in ll.iter().enumerate() {
    //     eprintln!("l[{i}] = {:?}", l);
    // }
    // for (i, r) in rr.iter().enumerate() {
    //     eprintln!("r[{i}] = {:?}", r);
    // }
    let mut rs = usize::MAX;
    for i in 0..(n - 1) {
        // i, i+1 文字目が 0のとき
        // eprintln!("[{i}] ll[{i}] rr[{}]", 2 + i);
        for zo in [0, 1] {
            // 例えば n=6 i=2 zo=0 のとき, 010010
            let mut cost = aa[i] * usize::from((s[i] as u8 - b'0') != zo as u8)
                + aa[i + 1] * usize::from((s[i + 1] as u8 - b'0') != zo as u8);
            cost += ll[i][(i + zo) % 2];
            cost += rr[2 + i][(n - i + zo) % 2];
            rs = rs.min(cost);
        }
    }
    println!("{rs}");
}
