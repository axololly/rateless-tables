use std::{collections::BTreeSet, fmt::Debug};

use crate::{AsBytes, AsVariableBytes, Bytes, Decoder, Encoder, Symbol};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u16,
    y: u16
}

impl Bytes for Point {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut padded = [0u8; 4];

        for (i, &byte) in bytes.iter().enumerate().take(4) {
            padded[i] = byte;
        }

        let x = u16::from_be_bytes(padded[..2].try_into().unwrap());
        let y = u16::from_be_bytes(padded[2..4].try_into().unwrap());

        Self { x, y }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        
        result.extend(self.x.to_be_bytes());
        result.extend(self.y.to_be_bytes());

        result
    }
}

fn slice_eq<T: Clone + Eq + Ord>(s1: &[T], s2: &[T]) -> bool {
    let h1: BTreeSet<T> = s1.iter().cloned().collect();
    let h2: BTreeSet<T> = s2.iter().cloned().collect();

    h1.symmetric_difference(&h2).next().is_none()
}

fn run_test<T: Symbol + Clone + Debug>(alice: Vec<T>, bob: Vec<T>) {
    let alice_needs_correct: Vec<T> = alice.iter().filter(|&v| !bob.contains(v)).cloned().collect();
    let bob_needs_correct: Vec<T> = bob.iter().filter(|&v| !alice.contains(v)).cloned().collect();

    let mut enc = Encoder::default();

    enc.extend(alice);

    let mut dec = Decoder::default();

    dec.extend(bob);

    let mut iters: usize = 0;

    for next in enc {
        iters += 1;

        if iters > 300 {
            panic!("exceeded 300 loops (something broke)");
        }
        
        dec.add_coded_symbol(next);

        dec.decode();
        
        if dec.is_done() {
            break;
        }
    }

    let (alice_needs, bob_needs) = dec.consume();
    
    assert!(slice_eq(&alice_needs, &alice_needs_correct));
    assert!(slice_eq(&bob_needs, &bob_needs_correct));
}

macro_rules! b {
    ($literal:expr) => {{
        AsBytes::new($literal)
    }};
}

macro_rules! vb {
    ($literal:expr) => {{
        AsVariableBytes::new($literal)
    }};
}

#[test]
fn variable_length_strings() {
    run_test(
        vec![
            vb!("hi".to_string()),
            vb!("hello".to_string()),
            vb!("another".to_string())
        ],

    vec![
            vb!("hello".to_string()),
            vb!("greetings".to_string())
        ]
    );
}

#[test]
fn fixed_length_types() {
    run_test(
        vec![
            b!(Point { x: 1, y: 2 }),
            b!(Point { x: 3, y: 3 })
        ],

        vec![
            b!(Point { x: 7, y: 0 }),
            b!(Point { x: 3, y: 3 })
        ]
    );
}

#[test]
fn variable_length_byte_arrays() {
    run_test(
        vec![
            vb!(b"hi".to_vec()),
            vb!(b"hello".to_vec()),
        ],

        vec![
            vb!(b"hello".to_vec()),
            vb!(b"greetings".to_vec()),
            vb!(b"another".to_vec())
        ]
    );
}

#[test]
fn variable_length_normal_ints() {
    run_test(
        vec![1, 2, 3, 4, 5, 6, 7, 9, 10],
        vec![1, 3, 4, 5, 6, 7, 8, 9, 10]
    );
}
