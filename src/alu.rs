use crate::basic_gates::*;
use crate::adder::*;

pub fn ALU(x: [bool; 16], y: [bool; 16], zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> ([bool; 16], bool, bool) {
    let a = Mux16(x, [false; 16], zx);
    let a = Mux16(x, Not16(x), nx);

    let b = Mux16(y, [false; 16], zy);
    let b = Mux16(y, Not16(y), ny);

    let out = Mux16(And16(x, y), Add16(x, y), f);
    let out = Mux16(out, Not16(out), no);

    let zr = Or(
        Or8Way([out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7]]),
        Or8Way([out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[14]]),
    );
    let ng = out[15];
    (out, zr, ng);
}
