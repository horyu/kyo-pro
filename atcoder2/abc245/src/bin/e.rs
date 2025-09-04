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
        aa: [usize; n],
        bb: [usize; n],
        cc: [usize; m],
        dd: [usize; m],
    };
    let aabb = izip!(aa, bb).sorted_unstable().collect_vec();
    let ccdd = izip!(cc, dd).collect_vec();

    let mm = btreemultimap::BTreeMultiMap::from_iter(ccdd);

    let mut bts = BTreeSet::new();
    let mut pre_a = usize::MAX;
    let mut tmp = 0;
    for (a, b) in aabb.into_iter().rev() {
        // 大きいaから見ていき、a以上のcのペアであるdを全部btsに追加し、そこから最小のdを取り出す
        for (_, &d) in mm.range(a..pre_a) {
            bts.insert((d, tmp));
            tmp += 1;
        }
        pre_a = a;

        if let Some(&k) = bts.range((b, 0)..).next() {
            bts.remove(&k);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
