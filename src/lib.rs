pub fn patience_argsort(v: Vec<u32>) -> Vec<u32> {
    let mut piles: Vec<Vec<u32>> = vec![vec![v[0]]]; // by index so we can do back-pointers.

    // For the backpointer we use the fact that we only ever append on the right, i.e. once a
    // column c exists the previous index will always be (c - 1), so we just need to store the
    // (c - 1).len() at the time of insertion.
    // TODO can probably do without option by just not using c == 0;
    let mut backpointer: Vec<Vec<Option<usize>>> = vec![vec![None]];

    for x in v.iter().skip(1) {
        match piles.binary_search_by(|probe| probe.last().unwrap().cmp(x)) {
            Ok(index) => {
                piles[index].push(*x);
                backpointer[index].push(if index < 1 {
                    None
                } else {
                    Some(piles[index - 1].len() - 1)
                });
            }
            Err(index) => {
                if piles.len() <= index {
                    piles.push(vec![*x]);
                    backpointer.push(vec![if index < 1 {
                        None
                    } else {
                        Some(piles[index - 1].len() - 1)
                    }]);
                } else {
                    piles[index].push(*x);
                    backpointer[index].push(if index < 1 {
                        None
                    } else {
                        Some(piles[index - 1].len() - 1)
                    });
                }
            }
        }
    }
    // Pick _a_ longest increasing subsequence, not necessarily unique
    let mut out: Vec<u32> = Vec::new();
    let mut i = backpointer.len() - 1;
    let mut j = 0;
    loop {
        out.push(piles[i][j]);
        match backpointer[i][j] {
            Some(new_j) => {
                j = new_j;
                i = i - 1;
            }
            None => break,
        }
    }
    out.reverse();
    out
}

// fn patience_diff(a: Vec<str>, b: Vec<str>) -> Vec<Occurence> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_argsort() {
        let v = vec![9, 13, 7, 12, 2, 1, 4, 6, 5, 8, 3, 11, 10];
        assert_eq!(patience_argsort(v), vec![1, 4, 5, 8, 11]);
        // let before = include_str!("testdata/before.c");
        // let after = include_str!("testdata/after.c");
    }
}
