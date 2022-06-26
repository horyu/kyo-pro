#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![allow(clippy::collapsible_else_if)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut aa: [isize; 3]
    };
    aa.sort_unstable();
    let rs = if aa[0] + aa[1] < aa[2] {
        -1
    } else if aa[2] - aa[1] <= aa[0] && aa[2] - aa[0] <= aa[1] {
        aa[2]
    } else {
        -1
    };
    println!("{}", rs);
}
