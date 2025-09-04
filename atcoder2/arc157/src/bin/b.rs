#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        s: Chars,
    };
    // https://atcoder.jp/contests/arc157/editorial/5789
    let mut ttff = s.into_iter().map(|c| c == 'Y').collect_vec();
    let x_count = ttff.iter().filter(|&&tf| !tf).count();
    if x_count == n {
        let rs = if k <= 1 { 0 } else { k - 1 };
        println!("{rs}");
        return;
    }
    if x_count < k {
        //XYを反転させ、 (n-x_count)個のxから (n-x_count)-(k-x_count)=n-k 個X→Yにすると考えられる
        ttff = ttff.into_iter().map(|tf| !tf).collect_vec();
        k = n - k;
    }
    let gg = ttff
        .iter()
        .copied()
        .group_by(|&tf| tf)
        .into_iter()
        .map(|(tf, group)| (tf, group.count()))
        .collect_vec();
    let m = gg.len();
    // 現在のカウント
    let mut rs = 0;
    // 内部のXXブロックサイズ
    let mut bb = vec![];
    for (i, (tf, c)) in gg.iter().copied().enumerate() {
        if tf {
            rs += c - 1;
        } else {
            if 0 < i && i < m - 1 {
                bb.push(c);
            }
        }
    }
    // 内部のXXブロックをサイズの小さい方からYYに変える
    bb.sort_unstable();
    for b in bb {
        if b <= k {
            rs += b + 1;
            k -= b;
        } else {
            rs += k;
            k = 0;
        }
        if k == 0 {
            break;
        }
    }
    // 外側のXXブロックを変える
    if m == 1 {
        rs += k.saturating_sub(1);
    } else {
        rs += k;
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        mut k: usize,
        s: Chars,
    };
    // ランレングス圧縮
    let mut rle = vec![];
    let mut ttff = vec![];
    let mut cnt = 1;
    for (ci, cj) in s.iter().tuple_windows() {
        if ci == cj {
            cnt += 1;
        } else {
            rle.push(cnt);
            ttff.push(ci == &'Y');
            cnt = 1;
        }
    }
    rle.push(cnt);
    ttff.push(s[n - 1] == 'Y');

    let m = rle.len();
    let mut counts = [0; 2];
    let mut rs = 0;
    for (&tf, &c) in izip!(&ttff, &rle) {
        if tf {
            rs += c - 1;
        }
        counts[tf as usize] += c;
    }

    if k <= counts[0] {
        // 内部のXXブロックをサイズの小さい方からYYに変える
        let mut bb = vec![];
        for (&tf, &c) in izip!(&ttff, &rle).skip(1).take(m.saturating_sub(2)) {
            if !tf {
                bb.push(c);
            }
        }
        bb.sort_unstable();
        for b in bb {
            if b <= k {
                rs += b + 1;
                k -= b;
            } else {
                rs += k;
                k = 0;
            }
            if k == 0 {
                println!("{rs}");
                return;
            }
        }
        // 外側のXXブロックを変える
        if m == 1 {
            rs += k.saturating_sub(1);
        } else {
            rs += k;
        }
        println!("{rs}");
        return;
    }

    // XXを全てYYに変えたとする
    k -= counts[0];
    for (i, (&tf, &c)) in izip!(&ttff, &rle).enumerate() {
        if tf {
            continue;
        }
        if i == 0 || i == m - 1 {
            rs += c;
        } else {
            rs += c + 1;
        }
    }
    dbg!(k, rs);
    if k == 0 {
        println!("{rs}");
        return;
    }
    // 外側のYYをXXに変える
    for i in [0, m - 1].into_iter().dedup() {
        if !ttff[i] {
            continue;
        }
        let c = rle[i];
        if c <= k {
            rs = rs.saturating_sub(c);
            k -= c;
        } else {
            rs = rs.saturating_sub(k);
            k = 0;
        }
        if k == 0 {
            println!("{rs}");
            return;
        }
    }
    // 内部の大きいYY→内部の小さいYYの順に変える
    let mut bb = vec![];
    for (&tf, &c) in izip!(&ttff, &rle).skip(1).take(m.saturating_sub(2)) {
        if tf {
            bb.push(c);
        }
    }
    bb.sort_unstable();
    for b in bb.into_iter().rev() {
        if b <= k {
            rs -= b + 1;
            k -= b;
        } else {
            rs -= k + 1;
            k = 0;
        }
        if k == 0 {
            println!("{rs}");
            return;
        }
    }

    println!("{rs}");
}
