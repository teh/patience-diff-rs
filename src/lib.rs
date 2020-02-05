pub fn patience_argsort<T: Ord + Copy>(v: Vec<T>) -> Vec<T> {
    if v.len() < 2 {
        return v;
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

// fn patience_diff(a: Vec<str>, b: Vec<str>) -> Vec<Occurence> {}

#[cfg(test)]
mod tests {
    use super::*;
    // Bring the macros and other important things into scope.
    use proptest::prelude::*;

    #[test]
    fn check_argsort() {
        let v = vec![9, 13, 7, 12, 2, 1, 4, 6, 5, 8, 3, 11, 10];
        assert_eq!(patience_argsort(v), vec![1, 4, 5, 8, 11]);
        // let before = include_str!("testdata/before.c");
        // let after = include_str!("testdata/after.c");
    }

    #[test]
    fn check_patience_sort_strings() {
        let v = vec!["a", "b", "f", "e", "c"];
        assert_eq!(patience_argsort(v), vec!["a", "b", "f"]);
    }


    proptest! {
        #[test]
        fn propcheck_argsort(v in prop::collection::vec(0u32..1_000, 0..10)) {
            patience_argsort(v);
        }
    }
}
