#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        // aa: [isize; 3]
        a1: isize,
        a2: isize,
        a3: isize,
    };
    let (a1, a3) = if a1 < a3 { (a1, a3) } else { (a3, a1) };
    let d21 = a2 - a1;
    let d31 = a3 - a1;
    if a2 < a1 + d31 / 2 {
        if d31.is_even() {
            println!("{}", a1 + d31 / 2 - a2);
        } else {
            println!("{}", a1 + d31 / 2 - a2 + 2); // a2, a3 に+1ずつ
        }
    } else {
        println!("{}", a2 + d21 - a3);
    }
}
