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

#[allow(clippy::identity_op, clippy::erasing_op)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };
    // 4s <= x <= 4s + 1:
    //   2t <= y <= 2t + 1: 1
    //   2t + 1 <= y <= 3t: 0.5
    // 4s + 1 <= x <= 4s + 2:
    //   2t <= y <= 2t + 0.5
    //   2t + 1 <= y <= 3t: 1
    // 4s + 2 <= x <= 4s + 3:
    //   2t <= y <= 2t + 1: 0
    //   2t + 1 <= y <= 3t: 0.5
    // 4s + 3 <= x <= 5s:
    //   2t <= y <= 2t + 1: 0.5
    //   2t + 1 <= y <= 3t: 0

    // (a, b, c, d) を 全て第1象限に移動
    let dx = a.abs().div_ceil(4) * 4;
    let dy = b.abs().div_ceil(4) * 4;
    let (a, b, c, d) = (a + dx, b + dy, c + dx, d + dy);
    eprintln!("{a} {b} {c} {d}");
    let mut rs = 0;
    // 4s <= x <= 4s + 1 の区間の数: (a-1)/4 <= s <= c/4 : c/4 - (a-1)/4 + 1
    //   2t <= y <= 2t + 1 の区間の数 d.div_ceil(2) - b.div_ceil(2)
    //   2t + 1 <= y <= 3t の区間の数 d / 2 - b / 2
    rs += ((c + 3) / 4 - (a + 3) / 4) * (2 * (d.div_ceil(2) - b.div_ceil(2)) + 1 * (d / 2 - b / 2));
    dbg!(rs);
    // 4s + 1 <= x <= 4s + 2 の区間の数: (a-2)/4 <= s <= (c-1)/4 : (c-1)/4 - (a-2)/4 + 1
    rs += ((c + 2) / 4 - (a + 2) / 4) * (1 * (d.div_ceil(2) - b.div_ceil(2)) + 2 * (d / 2 - b / 2));
    dbg!(rs);
    // 4s + 2 <= x <= 4s + 3 の区間の数: (a-3)/4 <= s <= (c-2)/4 : (c-2)/4 - (a-3)/4 + 1
    rs += ((c + 1) / 4 - (a + 1) / 4) * (0 * (d.div_ceil(2) - b.div_ceil(2)) + 1 * (d / 2 - b / 2));
    dbg!(rs);
    // 4s + 3 <= x <= 4s + 4 の区間の数: (a-4)/4 <= s <= (c-3)/4 : (c-3)/4 - (a-4)/4 + 1
    rs += (c / 4 - a / 4) * (1 * (d.div_ceil(2) - b.div_ceil(2)) + 0 * (d / 2 - b / 2));
    dbg!(rs);
    println!("{rs}");
}
