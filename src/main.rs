use ark_ff::{Field, PrimeField};

// We'll use a field associated with the BLS12-381 pairing-friendly
// group for this example.
use ark_bls12_381::Fq; // Prime Field

// `ark-std` is a utility crate that enables `arkworks` libraries
// to easily support `std` and `no_std` workloads, and also re-exports
// useful crates that should be common across the entire ecosystem, such as `rand`.
use ark_std::{One, UniformRand};

fn main() {
    let mut rng = ark_std::rand::thread_rng();
    // Let's sample uniformly random field elements:
    let a = Fq::rand(&mut rng);

    let modulus = <Fq as PrimeField>::MODULUS;

    // show that 1 + 1 = 2
    let one = Fq::one();
    assert_eq!(one + one, one.double());

    // show that the multiplicative inverse of a number multiplied by itself equals one.
    assert_eq!(a.inverse().expect("Should have an inverse") * a, one);

    // show that a value raised to the power of the modulus is equal to itself
    // use the `pow` function to raise to a power
    assert_eq!(a.pow(modulus), a); // the Euler-Fermat theorem
}
