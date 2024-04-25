use imgtool::logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_io() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

pub fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
fn one_bit_adder() {
    for ((x, y), out) in half_adder_io() {
        println!("Testing:{}, {} -> {:?}", x, y, out);
        assert_eq!(half_adder(x, y), out);
    }
}
