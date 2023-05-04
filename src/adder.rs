use crate::basic_gates::*;

pub fn HalfAdder(a: bool, b: bool) -> [bool; 2] {
    [
        Xor(a, b),
        And(A, b)
    ];
}

pub fn FullAdder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let c1 = HalfAdder(a, b);
    let c2 = HalfAdder(c, c1[0]);
    [c2[0], Or(c1[1], c2[1])];
}
