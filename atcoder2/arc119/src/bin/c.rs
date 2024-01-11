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
        aa: [isize; n],
    };
    /*
    12 8 11 3 3 13 2
    累積和っぽい何か？
    (l,r)=(2,4),(3,7),(4,5)
    (2,4) 8-11+3
    (3,7) 11-3+3-13+2
    (4,5) 3-3

    sum_odd[i]  := 要素i番目までで奇数番目の要素の累積和
    sum_even[i] := 要素i番目までで偶数番目の要素の累積和

    let ans = 0
    for i = 0 to n-2
        for j = i + 1 to n - 1
            if sum_odd[j]-sum_odd[i] == sum_even[j]-sum_even[i]
                ans += 1

    sum_odd[j]-sum_odd[i] == sum_even[j]-sum_even[i]を式変形すると
    sum_odd[j]-sum_even[j] == sum_odd[i]-sum_even[i]

    sum_odd[j]-sum_even[j] = sum_odd[i]-sum_even[i]となる(i, j)の組み合わせが答え
        */
    let mut sums = vec![[0; 2]; n + 1];
    for (i, a) in aa.iter().copied().enumerate() {
        sums[i + 1] = sums[i];
        if i % 2 == 0 {
            sums[i + 1][0] += a;
        } else {
            sums[i + 1][1] += a;
        }
    }
    let mut counter = counter::Counter::<_>::new();
    for i in 0..=n {
        counter[&(sums[i][0] - sums[i][1])] += 1;
    }
    let mut rs = 0;
    for (_, v) in counter {
        rs += v * v.saturating_sub(1) / 2;
    }
    println!("{rs}");
}
