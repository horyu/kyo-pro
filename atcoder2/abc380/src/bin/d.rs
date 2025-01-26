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

/*
S
ST
STTS
STTSTSST
STTSTSSTTSSTSTTS
*/
fn main() {
    input! {
        s: Chars,
        q: usize,
        kk: [Usize1; q],
    };
    let n = s.len();
    let mut rrss = vec![];
    for k in kk {
        let (mut div, rem) = k.div_rem(&n);
        let mut tf = true;
        while 0 < div {
            tf ^= true;
            div -= div.next_power_of_two() >> (!div.is_power_of_two() as u8);
        }
        let rs = if tf {
            s[rem]
        } else {
            (s[rem] as u8 ^ 0x20) as char
        };
        rrss.push(rs);
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
