use std::collections::HashSet;

pub fn sum(slice: &[i32]) -> i32 {
    let mut ret: i32 = 0;
    for i in slice {
        ret += i;
    }
    ret
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut books: HashSet<i32> = HashSet::new();
    let mut ret: Vec<i32> = Vec::new();

    for e in vs {
        if books.contains(e) {
            continue;
        }

        books.insert(*e);
        ret.push(*e);
    }

    ret
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();

    for e in vs {
        if pred(*e) {
            ret.push(*e);
        }
    }
    ret
}
