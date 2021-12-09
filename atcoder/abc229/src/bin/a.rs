#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s1: String,
        s2: String,
    };
    if (&s1 == ".#" && &s2 == "#.") || (&s1 == "#." && &s2 == ".#") {
        println!("No");
    } else {
        println!("Yes");
    }
}
