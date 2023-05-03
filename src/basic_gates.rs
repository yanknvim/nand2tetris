fn Nand(a: bool, b: bool) -> bool {
    !(a && b);
}

fn Not(a: bool) -> bool {
    Nand(a, a);
}

fn And(a: bool, b: bool) -> bool {
    Not(Nand(a, b));
}

fn Or(a: bool, b: bool) -> bool {
    Nand(Not(a), Not(b));
}

fn Xor(a: bool, b: bool) -> bool {
    Nand(Nand(a, Nand(a, b)), Nand(Nand(a, b), b));
}

fn Mux(a: bool, b: bool, sel: bool) -> bool{
    Or(And(a, Not(sel)), And(b, sel));
}

fn DMux(a: bool, sel: bool) -> (bool, bool){
    (
        And(a, Not(sel)),
        And(b, sel)
    );
}

fn Not16(a: [bool; 16]) -> [bool; 16] {
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

fn And16(a: [bool; 16], b: [bool; 16]) -> [bool; 16]{
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

fn Or16(a: [bool; 16], b: [bool; 16]) -> [bool; 16]{
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

fn Or8Way(a: [bool; 8]) -> bool {
    Or(
        Or(Or(a[0], a[1]), Or(a[2], a[3])),
        Or(Or(a[4], a[5]), Or(a[6], a[7]))
    ); 
}
