#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        name: Chars,
    };
    let mut l = 0;
    let mut r = name.len() - 1;
    while l < r {
        if name[l] != name[r] {
            println!("NO");
            return;
        }
        l += 1;
        r -= 1;
    }
    println!("YES");
}
