#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
    // カードの組み合わせは追加するたびに増える
    // 11 | (12,21),22 | (13,31,23,32),33
    // 既存の組み合わせ + 既存カードと新カードの組み合わせ + 新カードのみの組み合わせ
    let mut ft_cnt = ac_library::FenwickTree::new(2e5 as usize + 1, 0usize);
    let mut ft_sum = ac_library::FenwickTree::new(2e5 as usize + 1, 0usize);
    // 既存の組み合わせ
    let mut tmp = ModInt998244353::default();
    for (i, a) in aa.iter().copied().enumerate() {
        // 既存カードと新カードの組み合わせ
        // 1..=a 以下
        tmp += 2 * a * ft_cnt.sum(0..=a);
        // aとaより大きいカードの組み合わせ
        tmp += 2 * ft_sum.sum((a + 1)..);

        // 新カードのみの組み合わせ a.max(a) = a
        tmp += a;

        let rs = tmp / (i + 1).pow(2);
        println!("{rs}");
        ft_cnt.add(a, 1);
        ft_sum.add(a, a);
    }
}
