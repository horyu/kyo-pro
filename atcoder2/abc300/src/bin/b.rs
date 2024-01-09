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
        h: usize,
        w: usize,
        aaa: [Chars; h],
        mut bbb: [Chars; h],
    };
    for _ in 0..h {
        for _ in 0..w {
            if izip!(&aaa, &bbb).all(|(aa, bb)| aa == bb) {
                println!("Yes");
                return;
            }
            // rotate right
            for bb in bbb.iter_mut() {
                bb.rotate_right(1)
            }
        }
        // rotate doown
        bbb.rotate_right(1);
    }
    println!("No");
}
