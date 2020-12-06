#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let mae = is_month(&s[0..=1]);
    let ato = is_month(&s[2..=3]);
    println!(
        "{}",
        match (mae, ato) {
            (true, true) => "AMBIGUOUS",
            (true, false) => "MMYY",
            (false, true) => "YYMM",
            (false, false) => "NA",
        }
    );
}

fn is_month(s: &str) -> bool {
    matches!(
        s,
        "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" | "10" | "11" | "12"
    )
}
