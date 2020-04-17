use algebra_core::{
    biginteger::{BigInteger, BigInteger256},
    fields::{
        Field, Fp12Parameters, Fp2Parameters, Fp6Parameters, FpParameters, PrimeField,
        SquareRootField,
    },
    One, UniformRand, Zero,
};
use core::{
    cmp::Ordering,
    ops::{AddAssign, MulAssign, SubAssign},
};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::{
    bn_256::{Fq, Fq12, Fq12Parameters, Fq2, Fq2Parameters, Fq6, Fq6Parameters, FqParameters, Fr},
    tests::fields::{field_test, frobenius_test, primefield_test, sqrt_field_test},
};

pub(crate) const ITERATIONS: usize = 5;

#[test]
fn test_fr() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    for _ in 0..ITERATIONS {
        let a: Fr = UniformRand::rand(&mut rng);
        let b: Fr = UniformRand::rand(&mut rng);

        field_test(a, b);
        primefield_test::<Fr>();
        sqrt_field_test(a);
    }
}

#[test]
fn test_fq_one() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    for _ in 0..ITERATIONS {
        let a: Fq = UniformRand::rand(&mut rng);
        let b: Fq = UniformRand::rand(&mut rng);
        field_test(a, b);
        primefield_test::<Fq>();
        sqrt_field_test(a);
    }
}

#[test]
fn test_fq2() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    for _ in 0..ITERATIONS {
        let a: Fq2 = UniformRand::rand(&mut rng);
        let b: Fq2 = UniformRand::rand(&mut rng);
        field_test(a, b);
        sqrt_field_test(a);
    }
    frobenius_test::<Fq2, _>(Fq::characteristic(), 13);
}

#[test]
fn test_fq6() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    for _ in 0..ITERATIONS {
        let g: Fq6 = UniformRand::rand(&mut rng);
        let h: Fq6 = UniformRand::rand(&mut rng);
        field_test(g, h);
    }
    frobenius_test::<Fq6, _>(Fq::characteristic(), 13);
}

#[test]
fn test_fq12() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);
    for _ in 0..ITERATIONS {
        let g: Fq12 = UniformRand::rand(&mut rng);
        let h: Fq12 = UniformRand::rand(&mut rng);
        field_test(g, h);
    }
    frobenius_test::<Fq12, _>(Fq::characteristic(), 13);
}

//#[test]
fn test_negative_one() {
    let neg_one = Fq::new(BigInteger256([
        4332616871279656262u64,
        10917124144477883021u64,
        13281191951274694749u64,
        3486998266802970665u64,
    ]));
    assert_eq!(neg_one, -Fq::one());
}

#[test]
fn test_frob_coeffs() {
    let nqr = -Fq::one();

    assert_eq!(Fq2Parameters::FROBENIUS_COEFF_FP2_C1[0], Fq::one());
    assert_eq!(
        Fq2Parameters::FROBENIUS_COEFF_FP2_C1[1],
        nqr.pow([
            0x9e10460b6c3e7ea3,
            0xcbc0b548b438e546,
            0xdc2822db40c0ac2e,
            0x183227397098d014,
        ])
    );

    let nqr = Fq2::new(Fq::one(), Fq::one());

    assert_eq!(Fq6Parameters::FROBENIUS_COEFF_FP6_C1[0], Fq2::one());
}
