pub fn Nand(a: bool, b: bool) -> bool {
    !(a && b);
}

pub fn Not(a: bool) -> bool {
    Nand(a, a);
}

pub fn And(a: bool, b: bool) -> bool {
    Not(Nand(a, b));
}

pub fn Or(a: bool, b: bool) -> bool {
    Nand(Not(a), Not(b));
}

pub fn Xor(a: bool, b: bool) -> bool {
    Nand(Nand(a, Nand(a, b)), Nand(Nand(a, b), b));
}

pub fn Mux(a: bool, b: bool, sel: bool) -> bool{
    Or(And(a, Not(sel)), And(b, sel));
}

pub fn DMux(a: bool, sel: bool) -> [bool; 2]{
    [
        And(a, Not(sel)),
        And(b, sel)
    ];
}

pub fn Not16(a: [bool; 16]) -> [bool; 16] {
    [
        Not(a[0]),
        Not(a[1]),
        Not(a[2]),
        Not(a[3]),
        Not(a[4]),
        Not(a[5]),
        Not(a[6]),
        Not(a[7]),
        Not(a[8]),
        Not(a[9]),
        Not(a[10]),
        Not(a[11]),
        Not(a[12]),
        Not(a[13]),
        Not(a[14]),
        Not(a[15]),
    ];
}

pub fn And16(a: [bool; 16], b: [bool; 16]) -> [bool; 16]{
    [
        And(a[0], b[0]),
        And(a[1], b[1]),
        And(a[2], b[2]),
        And(a[3], b[3]),
        And(a[4], b[4]),
        And(a[5], b[5]),
        And(a[6], b[6]),
        And(a[7], b[7]),
        And(a[8], b[8]),
        And(a[9], b[9]),
        And(a[10], b[10]),
        And(a[11], b[11]),
        And(a[12], b[12]),
        And(a[13], b[13]),
        And(a[14], b[14]),
        And(a[15], b[15])
    ];
}

pub fn Or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16]{
    [
        Or(a[0], b[0]),
        Or(a[1], b[1]),
        Or(a[2], b[2]),
        Or(a[3], b[3]),
        Or(a[4], b[4]),
        Or(a[5], b[5]),
        Or(a[6], b[6]),
        Or(a[7], b[7]),
        Or(a[8], b[8]),
        Or(a[9], b[9]),
        Or(a[10], b[10]),
        Or(a[11], b[11]),
        Or(a[12], b[12]),
        Or(a[13], b[13]),
        Or(a[14], b[14]),
        Or(a[15], b[15])
    ];
}

pub fn Mux16(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
    [
        Mux(a[0], b[0], sel),
        Mux(a[1], b[1], sel),
        Mux(a[2], b[2], sel),
        Mux(a[3], b[3], sel),
        Mux(a[4], b[4], sel),
        Mux(a[5], b[5], sel),
        Mux(a[6], b[6], sel),
        Mux(a[7], b[7], sel),
        Mux(a[8], b[8], sel),
        Mux(a[9], b[9], sel),
        Mux(a[10], b[10], sel),
        Mux(a[11], b[11], sel),
        Mux(a[12], b[12], sel),
        Mux(a[13], b[13], sel),
        Mux(a[14], b[14], sel),
        Mux(a[15], b[15], sel)
    ];
}

pub fn Or8Way(a: [bool; 8]) -> bool {
    Or(
        Or(Or(a[0], a[1]), Or(a[2], a[3])),
        Or(Or(a[4], a[5]), Or(a[6], a[7]))
    ); 
}

pub fn Mux4Way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool: 16], sel: [bool; 2]) -> [bool; 16] {
    Mux16(Mux16(a, b, sel[0]), Mux16(c, d, sel[1]));
}

pub fn Mux8Way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], e: [bool; 16], f: [bool; 16], g: [bool; 16], h: [bool; 16], sel: [bool; 3]) -> [bool; 16] {
    Mux16(Mux4Way16(a, b, c, d, [sel[0], sel[1]]), Mux4Way16(e, f, g, h [sel[0], sel[1]]), sel[2]);
}

pub fn DMux4Way(input: bool, sel: [bool; 2]) -> [bool; 4] {
    let [ao, bo] = DMux(input, sel[1]);
    let [a, b] = DMux(ao, sel[0]);
    let [c, d] = DMux(bo, sel[0]);
    [a, b, c, d];
}

pub fn DMux8Way(input: bool, sel: [bool; 3]) -> [bool; 8] {
    let [ao, bo] = DMux(input, sel[2]);
    let [a, b, c, d] = DMux4Way(ao, [sel[0], sel[1]]);
    let [e, f, g, h] = DMux4Way(bo, [sel[0], sel[1]]);
    [a, b, c, d, e, f, g, h];
}
