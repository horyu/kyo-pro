#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

/*
[1:1]
1 = 0+(1) : 1+(0)
[1:2]
21
 1
22 = 1+(1,1) : 2+(0,0)
[1:3]
321
 21
  1
343 = 2+(1,2,1) : 3+(0,1,0)
[1:4]
4321
 321
  21
   1
4664 = 3+(1,3,3,1) : 4+(0,2,2,0)
[1:5]
54321
 4321
  321
   21
    1
58985 = 4+(1,4,5,4,1) : 5+(0,3,4,3,0)
[1:6]
654321
 54321
  4321
   321
    21
     1
602206 = 5+(1,5,7,7,5,1) : 6+(0,4,6,6,4,0)

ans(i:j) = ??
*/
fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        llrr: [(Usize1, Usize1); q],
    };
    let mut memo = HashMap::new();
    let mut ft = ac_library::FenwickTree::new(n, 0usize);
    for (i, a) in aa.iter().copied().enumerate() {
        ft.add(i, a);
    }
    for (l, r) in llrr {
        let rs = (r - l) * ft.sum(l..=r) + f(l, r, &aa, &mut memo);
        println!("{rs}");
    }
}

fn f(
    l: usize,
    r: usize,
    aa: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&v) = memo.get(&(l, r)) {
        return v;
    }
    let v = if l == r {
        aa[l]
    } else {
        f(l, r - 1, aa, memo) + f(l + 1, r, aa, memo)
    };
    memo.insert((l, r), v);
    v
}
