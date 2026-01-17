use rateless_tables::{Decoder, Encoder};

fn main() {
    let mut alice = Encoder::default();

    alice.extend([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let mut bob = Decoder::default();

    bob.extend([1, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    let mut iters = 0;

    for next in alice {
        iters += 1;

        bob.add_coded_symbol(next);

        bob.decode();

        if bob.is_done() {
            break;
        }
    }

    let (alice_needs, bob_needs) = bob.consume();

    println!("iterations: {iters}");
    println!("alice needs: {alice_needs:?}");
    println!("bob needs: {bob_needs:?}");

    println!("done!");
}
