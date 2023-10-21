#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(exclusive_range_pattern)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        cg: usize,
        is: f64,
    };
    let w = match (is / 60.0 * 10.0).round() as usize {
        0..3 => 0,
        3..16 => 1,
        16..34 => 2,
        34..55 => 3,
        55..80 => 4,
        80..108 => 5,
        108..139 => 6,
        139..172 => 7,
        172..208 => 8,
        208..245 => 9,
        245..285 => 10,
        285..327 => 11,
        _ => 12,
    };
    if w == 0 {
        println!("C 0");
        return;
    }
    let ir = match cg * 10 {
        1125..3375 => "NNE",
        3375..5625 => "NE",
        5625..7875 => "ENE",
        7875..10125 => "E",
        10125..12375 => "ESE",
        12375..14625 => "SE",
        14625..16875 => "SSE",
        16875..19125 => "S",
        19125..21375 => "SSW",
        21375..23625 => "SW",
        23625..25875 => "WSW",
        25875..28125 => "W",
        28125..30375 => "WNW",
        30375..32625 => "NW",
        32625..34875 => "NNW",
        _ => "N",
    };
    println!("{ir} {w}");
}
