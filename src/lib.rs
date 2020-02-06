// Sources:
// * https://blog.jcoglan.com/2017/09/19/the-patience-diff-algorithm/
// * https://bramcohen.livejournal.com/73318.html
// * https://alfedenzo.livejournal.com/170301.html

use std::collections::hash_map::Entry;
use std::collections::HashMap;

enum UniqueCheck {
    Line(usize),
    Duplicated,
}

fn longest_increasing_subsequence<T: Ord + Copy>(v: &Vec<T>) -> Vec<T> {
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

fn unique_check<T, I>(iter: I, offset: usize) -> HashMap<T, UniqueCheck>
where
    T: Ord + Eq + Copy + std::hash::Hash + std::fmt::Debug,
    I: Iterator<Item = T>,
{
    let mut unique_map: HashMap<T, UniqueCheck> = HashMap::new();
    for (ix, x) in iter.enumerate() {
        match unique_map.entry(x) {
            Entry::Vacant(xe) => {
                xe.insert(UniqueCheck::Line(ix + offset));
            }
            Entry::Occupied(mut xe) => {
                let xem = xe.get_mut();
                if let UniqueCheck::Line(_) = xem {
                    *xem = UniqueCheck::Duplicated
                }
            }
        }
    }
    unique_map
}

pub fn patience_diff<T: Ord + Eq + Copy + std::hash::Hash + std::fmt::Debug>(a: Vec<T>, b: Vec<T>) {
    let an = a.len();
    let bn = b.len();
    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, 0, an, bn)];

    while let Some((mut a0, mut b0, mut a1, mut b1)) = queue.pop() {
        // 1. Walk from start and end until mismatch. This removes
        //    code common to both a and b.
        while a0 < a1 && b0 < b1 && a[a0] == b[b0] {
            a0 += 1;
            b0 += 1;
        }
        while a1 > a0 && b1 > b0 && a[a1 - 1] == b[b1 - 1] {
            a1 -= 1;
            b1 -= 1;
        }

        // 2. find matching uniques, keeping line numbers. We're using an enum
        // to keep track of the line number, and set it to Duplicated if already
        // in the map.
        let a_map = unique_check(a[a0..a1].iter(), a0);
        let b_map = unique_check(b[b0..b1].iter(), b0);

        let mut rhs: Vec<(usize, usize)> = Vec::new();
        for (ix, x) in a[a0..a1].iter().enumerate() {
            if a_map.contains_key(x) && b_map.contains_key(x) {
                match b_map.get(x) {
                    Some(UniqueCheck::Line(z)) => {
                        // somewhat unintuitive: We use tuples of (right-side, left-size), that
                        // way the Ord trait works correctly in longest_increasing_subsequence later.
                        rhs.push((*z, a0 + ix));
                    }
                    _ => {}
                }
            }
        }
        let rhs2 = longest_increasing_subsequence(&rhs);
        if rhs2.is_empty() {
            // TODO somehow transform the following into a diff structure.
            println!(
                "---\n{:?}\n+++\n{:?}",
                a[a0..a1].to_vec(),
                b[b0..b1].to_vec()
            );
        } else {
            let start = vec![(b0, a0)];
            let end = vec![(b1, a1)];
            let together = start.iter().chain(rhs2.iter()).chain(end.iter());

            // note that a and b are flipped because of the reversed tuple used in partience_argsort.
            for ((b_start, a_start), (b_end, a_end)) in together.clone().zip(together.skip(1)) {
                println!(
                    "a: {:?}->{:?}, b: {:?}->{:?}",
                    a_start, a_end, b_start, b_end
                );
                queue.push((*a_start, *b_start, *a_end, *b_end));
            }
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
        let before = vec!["x", "y", "c", "z", "0"];
        let after = vec!["x", "b", "y", "z", "1"];
        patience_diff(before, after);
    }

    #[test]
    fn check_argsort() {
        let v = vec![9, 13, 7, 12, 2, 1, 4, 6, 5, 8, 3, 11, 10];
        assert_eq!(longest_increasing_subsequence(&v), vec![1, 4, 5, 8, 11]);
    }

    #[test]
    fn check_patience_sort_strings() {
        let v = vec!["a", "b", "f", "e", "c"];
        assert_eq!(longest_increasing_subsequence(&v), vec!["a", "b", "f"]);
    }

    proptest! {
        #[test]
        fn propcheck_argsort(v in prop::collection::vec(0u32..1_000, 0..10)) {
            longest_increasing_subsequence(&v);
        }
    }
}
