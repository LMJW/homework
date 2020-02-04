#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Peg {
    A,
    B,
    C,
}

pub type Move = (Peg, Peg);

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut ret: Vec<Move> = Vec::new();
    if num_discs == 0 {
        return ret;
    } else if num_discs == 1 {
        ret.push((src, dst));
        return ret;
    } else if num_discs == 2 {
        ret.push((src, aux));
        ret.push((src, dst));
        ret.push((aux, dst));
        return ret;
    } else {
        let mut r1 = hanoi(num_discs - 1, src, dst, aux);
        r1.push((src, dst));
        let mut r2 = hanoi(num_discs - 1, aux, src, dst);
        r1.extend(r2);
        return r1;
    }
}
