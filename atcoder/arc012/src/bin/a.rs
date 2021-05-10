#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        day: String
    };
    let rs = match day.as_str() {
        "Sunday" => 0,
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        "Saturday" => 0,
        _ => unreachable!(),
    };
    println!("{}", rs);
}
