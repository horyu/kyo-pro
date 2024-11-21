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
        hh: [usize; n],
    };
    let mut t = 0;
    for h in hh {
        let (div, mut rem) = h.div_rem(&5);
        t += div * 3;
        while 0 < rem {
            t += 1;
            if t % 3 == 0 {
                rem = rem.saturating_sub(3);
            } else {
                rem -= 1;
            }
        }
    }
    let rs = t;
    println!("{rs}");
}
