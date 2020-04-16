use crate::{
    biginteger::BigInteger256,
    bn_256,
    bn_256::*,
    curves::{
        bn,
        models::{ModelParameters, SWModelParameters},
    },
    field_new, Zero,
};

pub type G1Affine = bn::G1Affine<bn_256::Parameters>;
pub type G1Projective = bn::G1Projective<bn_256::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// TODO
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, BigInteger256([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]));

    /// TODO
    const COEFF_B: Fq2 = field_new!(
        Fq2,
        field_new!(
            Fq,
            BigInteger256([
                0x3bf938e377b802a8,
                0x020b1b273633535d,
                0x26b7edf049755260,
                0x2514c6324384a86d,
            ])
        ),
        field_new!(
            Fq,
            BigInteger256([
                0x38e7ecccd1dcff67,
                0x65f0b37d93ce0d3e,
                0xd749d0dd22ac00aa,
                0x0141b9ce4a688d4d,
            ])
        ),
    );

    /// TODO
    const COFACTOR: &'static [u64] = &[
        0x345f2299c0f9fa8d,
        0x06ceecda572a2489,
        0xb85045b68181585e,
        0x30644e72e131a029,
    ];

    /// TODO
    /// COFACTOR_INV = COFACTOR^{-1} mod r
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, BigInteger256([
        0x0,
        0x0,
        0x0,
        0x0,
    ]));

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

// Generator of G1
// x = 1
// y = 2
pub const G1_GENERATOR_X: Fq = field_new!(
    Fq,
    BigInteger256([
        0xd35d438dc58f0d9d,
        0x0a78eb28f5c70b3d,
        0x666ea36f7879462c,
        0x0e0a77c19a07df2f,
    ])
);

pub const G1_GENERATOR_Y: Fq = field_new!(
    Fq,
    BigInteger256([
        0xa6ba871b8b1e1b3a,
        0x14f1d651eb8e167b,
        0xccdd46def0f28c58,
        0x1c14ef83340fbe5e,
    ])
);
