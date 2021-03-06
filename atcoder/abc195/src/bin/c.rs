#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: u128
    };
    let rs = if n == 1_000_000_000_000_000 {
        5 + 4 * (1_000_000_000_000_000 - 1_000_000_000_000)
            + 3 * (1_000_000_000_000 - 1_000_000_000)
            + 2 * (1_000_000_000 - 1_000_000)
            + (1_000_000 - 1_000)
    } else if 1_000_000_000_000 <= n {
        4 * (n - 1_000_000_000_000 + 1)
            + 3 * (1_000_000_000_000 - 1_000_000_000)
            + 2 * (1_000_000_000 - 1_000_000)
            + (1_000_000 - 1_000)
    } else if 1_000_000_000 <= n {
        3 * (n - 1_000_000_000 + 1) + 2 * (1_000_000_000 - 1_000_000) + (1_000_000 - 1_000)
    } else if 1_000_000 <= n {
        2 * (n - 1_000_000 + 1) + (1_000_000 - 1_000)
    } else if 1_000 <= n {
        n - 1_000 + 1
    } else {
        0
    };
    println!("{}", rs);
}
