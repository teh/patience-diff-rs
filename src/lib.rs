// Sources:
// * https://blog.jcoglan.com/2017/09/19/the-patience-diff-algorithm/
// * https://bramcohen.livejournal.com/73318.html
// * https://alfedenzo.livejournal.com/170301.html

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::BitAnd;


pub fn patience_argsort<T: Ord + Copy>(v: &Vec<T>) -> Vec<T> {
    if v.len() < 2 {
        return v.clone();
    }

    let mut piles: Vec<Vec<T>> = vec![vec![v[0]]]; // by index so we can do back-pointers.
    let mut out: Vec<T> = Vec::new();

    // For the backpointer we use the fact that we only ever append on the right, i.e. once a
    // column c exists the previous index will always be (c - 1), so we just need to store
    // (c - 1).len() at the time of insertion.
    // Note that we don't keep backpointers for the first heap (see index > 0 check below)
    let mut backpointer: Vec<Vec<usize>> = vec![];

    for x in v.iter().skip(1) {
        let index = match piles.binary_search_by(|probe| probe.last().unwrap().cmp(x)) {
            Ok(index) => index,
            Err(index) => index,
        };
        if piles.len() <= index {
            piles.push(vec![*x]);
            backpointer.push(vec![piles[index - 1].len() - 1]);
        } else {
            piles[index].push(*x);
            if index > 0 {
                backpointer[index - 1].push(piles[index - 1].len() - 1);
            }
        }
    }
    // Pick _a_ longest increasing subsequence, not necessarily unique
    let mut i = piles.len() - 1;
    let mut j = 0;
    while i > 0 {
        out.push(piles[i][j]);
        j = backpointer[i - 1][j];
        i = i - 1;
    }
    out.push(piles[i][j]);
    out.reverse();
    out
}

fn patience_diff<T: Ord + Eq + Copy + std::hash::Hash + std::fmt::Debug>(a: Vec<T>, b: Vec<T>) {
    let an = a.len();
    let bn = b.len();

    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, 0, an, bn)];
    println!("q {:?}", queue);

    while let Some((mut h, mut i, mut j, mut k)) = queue.pop() {
        // 1. walk from start, end until mismatch
        // TODO naming of variables is bad. maybe use ai, aj, bi, bj?
        println!("X h: {}, i: {}, j: {}, k: {}", h, i, j, k);
        let (sh, si, sj, sk) = (h, i, j, k);
        while h < j && i < k && a[i] == b[i] {
            h += 1;
            i += 1;
        }
        while j > h && k > i && a[j - 1] == b[k - 1] {
            j -= 1;
            k -= 1;
        }
        println!("  h: {}, i: {}, j: {}, k: {}", h, i, j, k);
        // anything from (si..i) and (j..sj), (k..sk) is the same and can be kept.

        // 2. find matching uniques, keeping line numbers. We're using Some(i)
        // to keep a line number, and None to mark duplicates. Should probably
        // be its own enum..
        let mut a_map: HashMap<T, Option<usize>> = HashMap::new();
        let mut b_map: HashMap<T, Option<usize>> = HashMap::new();

        for (ix, x) in a[h..j].iter().enumerate() {
            match a_map.entry(*x) {
                Entry::Vacant(xe) => {
                    xe.insert(Some(ix + h));
                }
                Entry::Occupied(mut xe) => {
                    let xem = xe.get_mut();
                    if xem.is_some() {
                        *xem = None
                    }
                }
            }
        }

        // TODO factor out into function
        for (ix, x) in b[i..k].iter().enumerate() {
            match b_map.entry(*x) {
                Entry::Vacant(xe) => {
                    xe.insert(Some(ix + i));
                }
                Entry::Occupied(mut xe) => {
                    let xem = xe.get_mut();
                    if xem.is_some() {
                        *xem = None
                    }
                }
            }
        }

        let mut rhs: Vec<(usize, usize)> = Vec::new();
        for (ix, x) in a[h..j].iter().enumerate() {
            if a_map.contains_key(x) && b_map.contains_key(x) {
                match b_map.get(x) {
                    Some(Some(z)) => {
                        rhs.push((*z, h + ix));
                    }
                    _ => {}
                }
            }
        }
        let rhs2 = patience_argsort(&rhs);
        if rhs2.is_empty() {
            println!("---\n{:?}\n+++\n{:?}", a[h..j].to_vec(), b[i..k].to_vec());
        } else {
            println!("recurse {:?}", rhs2);
            // we know rhs[0].1 left matches rhs[0].0 right (swapped due to patience sort)
            queue.push((h, i, rhs2[0].1, rhs2[0].0));
            queue.push((rhs2[0].1 + 1, rhs2[0].0 + 1, sj, sk));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Bring the macros and other important things into scope.
    use proptest::prelude::*;

    #[test]
    fn check_diff() {
        // let before = include_str!("testdata/before.c").lines().collect();
        // let after = include_str!("testdata/after.c").lines().collect();
        let before = vec!["x", "y", "c"];
        let after = vec!["x", "b", "y"];
        patience_diff(before, after);
    }

    #[test]
    fn check_argsort() {
        let v = vec![9, 13, 7, 12, 2, 1, 4, 6, 5, 8, 3, 11, 10];
        assert_eq!(patience_argsort(&v), vec![1, 4, 5, 8, 11]);
    }

    #[test]
    fn check_patience_sort_strings() {
        let v = vec!["a", "b", "f", "e", "c"];
        assert_eq!(patience_argsort(&v), vec!["a", "b", "f"]);
    }

    proptest! {
        #[test]
        fn propcheck_argsort(v in prop::collection::vec(0u32..1_000, 0..10)) {
            patience_argsort(&v);
        }
    }
}
