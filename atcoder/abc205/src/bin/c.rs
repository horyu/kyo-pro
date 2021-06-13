#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut a: isize,
        mut b: isize,
        c: u32,
    };
    use std::cmp::Ordering;
    if c.is_even() {
        a = a.abs();
        b = b.abs();
    }
    println!(
        "{}",
        match a.cmp(&b) {
            Ordering::Less => "<",
            Ordering::Equal => "=",
            Ordering::Greater => ">",
        }
    );
}
