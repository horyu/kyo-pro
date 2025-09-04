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
        m: usize,
        k: usize,
    };
    // https://atcoder.jp/contests/abc341/editorial/9334
    let kk = k - 1;
    let min = n.min(m);
    let max = n.max(m);
    let lcm = n.lcm(&m);
    // 1..=lcm までの間にmin,maxの片方せしか割り切れない数
    let cnt = lcm / min - 1 + lcm / max - 1;
    // let cnt = if max == lcm {
    //     lcm / min - 1
    // } else {
    //     lcm / min - 1 + lcm / max - 1
    // };
    dbg!(lcm, cnt);
    dbg!(kk / cnt);
    dbg!(k - kk / cnt * cnt);
    let mut arr = [min, max];
    for _ in 0..(kk % cnt) {
        if arr[0] < arr[1] {
            arr[0] += min;
        } else {
            arr[1] += max;
        }
    }
    let rs = kk / cnt * lcm + arr.iter().copied().min().unwrap();
    println!("{rs}");
}
