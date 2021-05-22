#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    if n.is_even() {
        println!("NO");
        return;
    }
    let tf = (3..=n.sqrt()).step_by(2).all(|num| n % num != 0);
    println!("{}", ["NO", "YES"][tf as usize])
}
