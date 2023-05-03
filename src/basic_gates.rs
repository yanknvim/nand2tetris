fn Nand(a: bool, b: bool) -> bool {
    return !(a && b);
}

fn Not(a: bool) -> bool {
    return Nand(a, a);
}

fn And(a: bool, b: bool) -> bool {
    return Not(Nand(a, b));
}

fn Or(a: bool, b: bool) -> bool {
    return Nand(Not(a), Not(b));
}

fn Xor(a: bool, b: bool) -> bool {
    return Nand(Nand(a, Nand(a, b)), Nand(Nand(a, b), b));
}

fn Mux(a: bool, b: bool, sel: bool) -> bool{
    return Or(And(a, Not(sel)), And(b, sel));
}

fn DMux(a: bool, sel: bool) -> (bool, bool){
    return (
        And(a, Not(sel)),
        And(b, sel)
    );
}

fn Not16(a: [bool; 16]) -> [bool; 16] {
    return [
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

fn And16(a: [bool; 16], b: [bool: 16]) {

}
