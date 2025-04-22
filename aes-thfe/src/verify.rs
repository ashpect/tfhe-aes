#![allow(dead_code)]
use tfhe::FheBool;

#[derive(Copy, Clone, Debug)]
enum ParityMode {
    // The sum bits of message + parity bit must an odd number
    Odd,
    // The sum bits of message + parity bit must an even number
    Even,
}

fn compute_parity_bit(fhe_bits: &[FheBool], mode: ParityMode) -> FheBool {
    let mut parity_bit = fhe_bits[0].clone();
    for fhe_bit in &fhe_bits[1..] {
        parity_bit = fhe_bit ^ parity_bit
    }

    match mode {
        ParityMode::Odd => !parity_bit,
        ParityMode::Even => parity_bit,
    }
}

fn is_even(n: u8) -> bool {
    (n & 1) == 0
}

fn is_odd(n: u8) -> bool {
    !is_even(n)
}

fn check_parity_bit_validity(bits: &[bool], mode: ParityMode, parity_bit: bool) -> bool {
    let num_bit_set = bits
        .iter()
        .map(|bit| *bit as u8)
        .fold(parity_bit as u8, |acc, bit| acc + bit);

    match mode {
        ParityMode::Even => is_even(num_bit_set),
        ParityMode::Odd => is_odd(num_bit_set),
    }
}
