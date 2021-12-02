#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let ng = "UNRESTORABLE";

    let slen = s.len();
    let tlen = t.len();
    if slen < tlen {
        println!("{}", ng);
        return;
    }
    // 辞書順なのでsの後ろからマッチしないか探す
    for i in 0..=(slen - tlen) {
        if t.iter()
            .zip(s[(slen - tlen - i)..(slen - i)].iter())
            .all(|(tc, sc)| tc == sc || '?' == *sc)
        {
            let mut rs = String::new();
            for &sc in &s[0..slen - tlen - i] {
                rs.push(if sc == '?' { 'a' } else { sc });
            }
            for tc in t {
                rs.push(tc);
            }
            for &sc in &s[(slen - i)..] {
                rs.push(if sc == '?' { 'a' } else { sc });
            }
            println!("{}", rs);
            return;
        }
    }
    println!("{}", ng);
}
