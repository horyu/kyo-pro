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
        aa: [usize; n],
    };
    let mut rs = 0;
    // 尺取法
    for start in 0..(2.min(n)) {
        let mut l = start;
        let mut sum = 0;
        let mut counter = counter::Counter::<_>::new();
        for r in (start..(n - 1)).step_by(2) {
            if aa[r] == aa[r + 1] {
                counter[&aa[r]] += 2;
                sum += 2;
                // 2個になるように縮める
                while counter[&aa[r]] == 4 {
                    counter[&aa[l]] -= 2;
                    sum -= 2;
                    l += 2;
                }
                rs = rs.max(sum);
            } else {
                // リセット
                counter.clear();
                sum = 0;
                l = r + 2;
            }
        }
    }
    println!("{rs}");
}
