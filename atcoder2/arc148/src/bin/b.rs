#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let string = s.iter().join("");
    let str = string.as_str();
    // 先頭のdを伸ばせるだけ伸ばす
    if let Some(l) = s.iter().position(|&c| c == 'p') {
        let mut min = string.as_str()[l..].to_owned();
        for r in l..n {
            min = min.min(format!(
                "{}{}",
                s[l..=r]
                    .iter()
                    .rev()
                    .map(|&c| if c == 'd' { 'p' } else { 'd' })
                    .join(""),
                &str[(r + 1)..]
            ));
        }
        println!("{}{}", &str[..l], min);
    } else {
        let rs = s.iter().join("");
        println!("{rs}");
    };
}
