use std::collections::HashMap;
use std::iter::zip;

//function to get stats of tokens
pub fn get_stats(ids: &[i32]) -> HashMap<(i32, i32), i32> {
    let mut stats = HashMap::new();
    for pair in zip(ids, &ids[1..]) {
        *stats.entry((*pair.0, *pair.1)).or_insert(0) += 1;
    }
    stats
}

//function to merge tokens
pub fn merge(ids: &[i32], pair: (i32, i32), idx: i32) -> Vec<i32> {
    let mut newids = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        // if not at the very last position AND the pair matches, replace it
        if ids[i] == pair.0 && i < ids.len() - 1 && ids[i + 1] == pair.1 {
            newids.push(idx);
            i += 2;
        } else {
            newids.push(ids[i]);
            i += 1;
        }
    }
    newids
}
