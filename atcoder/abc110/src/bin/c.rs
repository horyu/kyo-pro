#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ts = [[false; 26]; 26];
    let mut st = [[false; 26]; 26];
    for (&sc, &tc) in s.iter().zip(t.iter()) {
        let ti = (tc as u8 - b'a') as usize;
        let si = (sc as u8 - b'a') as usize;
        ts[ti][si] = true;
        st[si][ti] = true;
    }
    let tf = ts.iter().all(|arr| arr.iter().filter(|&&x| x).count() <= 1)
    && st.iter().all(|arr| arr.iter().filter(|&&x| x).count() <= 1);
    println!("{}", ["No", "Yes"][tf as usize]);
}
