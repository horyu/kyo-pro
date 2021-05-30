#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut abc: [usize; 3]
    };
    abc.sort();
    println!(
        "{}",
        if abc[0] == abc[1] {
            abc[2]
        } else if abc[1] == abc[2] {
            abc[0]
        } else {
            0
        }
    );
}
