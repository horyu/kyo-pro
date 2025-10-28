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
1 = 0+(1)
[1:2]
21
 1
22 = 1+(1,1)
[1:3]
321
 21
  1
343 = 2+(1,2,1)
[1:4]
4321
 321
  21
   1
4664 = 3+(1,3,3,1)

f(x,y)をパスカルの三角形のx行y列の値とする
ans = sum_{i=l}^{r} aa[i] * ((r-l) + f(r-l,i-l))
*/
fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        llrr: [(Usize1, Usize1); q],
    };
    // TODO: 高速化　(l,r)=(1,n) q個でTLE
    // println!("{rs}");
}
