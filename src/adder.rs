use crate::basic_gates::*;

pub fn HalfAdder(a: bool, b: bool) -> [bool; 2] {
    [
        Xor(a, b),
        And(a, b)
    ]
}

pub fn FullAdder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let c1 = HalfAdder(a, b);
    let c2 = HalfAdder(c, c1[0]);
    [c2[0], Or(c1[1], c2[1])]
}

pub fn Add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let adder1 = HalfAdder(a[0], b[0]);
    let adder2 = FullAdder(a[1], b[1], adder1[1]);
    let adder3 = FullAdder(a[2], b[2], adder2[1]);
    let adder4 = FullAdder(a[3], b[3], adder3[1]);
    let adder5 = FullAdder(a[4], b[4], adder4[1]);
    let adder6 = FullAdder(a[5], b[5], adder5[1]);
    let adder7 = FullAdder(a[6], b[6], adder6[1]);
    let adder8 = FullAdder(a[7], b[7], adder7[1]);
    let adder9 = FullAdder(a[8], b[8], adder8[1]);
    let adder10 = FullAdder(a[9], b[9], adder9[1]);
    let adder11 = FullAdder(a[10], b[10], adder10[1]);
    let adder12 = FullAdder(a[11], b[11], adder11[1]);
    let adder13 = FullAdder(a[12], b[12], adder12[1]);
    let adder14 = FullAdder(a[13], b[13], adder13[1]);
    let adder15 = FullAdder(a[14], b[14], adder14[1]);
    let adder16 = FullAdder(a[15], b[15], adder15[1]);
    [
        adder1[0], adder2[0], adder3[0], adder4[0], adder5[0], adder6[0], adder7[0], adder8[0],
        adder9[0], adder10[0], adder11[0], adder12[0], adder13[0], adder14[0], adder15[0], adder16[0]
    ]
}

fn Inc16(a: [bool; 16]) -> [bool; 16] {
    Add16(a, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false])
}
