#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
 
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    };
    s.sort();
    t.sort();
    t.reverse();
    let tf = (0..s.len()).any(|index| (index < t.len()) && (s[index] < t[index]))
        || ((s.len() < t.len()) && (0..s.len()).all(|index| s[index] == t[index]));
    println!("{}", ["No", "Yes"][tf as usize]);
}
