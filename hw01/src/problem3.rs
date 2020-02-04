use std::convert::TryInto;

pub fn sieve(n: u32) -> Vec<u32> {
    let nn = n as usize;
    let mut a = vec![true; nn];
    let m = n as f64;
    let m = (m.sqrt() + 1.) as usize;

    for i in 2..m {
        if a[i] {
            let mut j = i * i;
            while j < nn {
                a[j] = false;
                j = j + i;
            }
        }
    }
    let mut ret: Vec<u32> = Vec::new();
    for (i, v) in a.into_iter().enumerate() {
        if v {
            ret.push(i.try_into().unwrap());
        }
    }

    ret[2..].to_vec()
}
