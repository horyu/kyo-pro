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
        k: usize,
        s: Chars,
        t: Chars,
    };
    if 1 < s.len().abs_diff(t.len()) {
        println!("No");
        return;
    }
    let (s, t) = if s.len() < t.len() { (s, t) } else { (t, s) };
    let tf = if s.len() == t.len() {
        izip!(s.iter(), t.iter())
            .filter(|(&sc, &tc)| sc != tc)
            .count()
            <= 1
    } else {
        // sに任意の1文字を追加 = tの任意の1文字を削除
        let mut i = 0;
        let mut j = 0;
        let mut diff = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
                diff += 1;
            }
        }
        diff + s.len() - i <= 1
    };
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
