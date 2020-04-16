use crate::bn_256::*;
use algebra_core::{biginteger::BigInteger256 as BigInteger, field_new, fields::*};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// (Not used)
    const NONRESIDUE: Fq = field_new!(Fq, BigInteger([0x0, 0x0, 0x0, 0x0,]));

    /// (Not used)
    /// QUADRATIC_NONRESIDUE = (U + 9)
    const QUADRATIC_NONRESIDUE: (Fq, Fq) = (
        field_new!(Fq, BigInteger([0x0, 0x0, 0x0, 0x0,])),
        field_new!(Fq, BigInteger([0x0, 0x0, 0x0, 0x0,])),
    );

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: [Fq; 2] = [
        // Fq(-1)**(((q^0) - 1) / 2)
        field_new!(
            Fq,
            BigInteger([
                0xd35d438dc58f0d9d,
                0x0a78eb28f5c70b3d,
                0x666ea36f7879462c,
                0x0e0a77c19a07df2f,
            ])
        ),
        // Fq(-1)**(((q^1) - 1) / 2)
        field_new!(
            Fq,
            BigInteger([
                0x68c3488912edefaa,
                0x8d087f6872aabf4f,
                0x51e1a24709081231,
                0x2259d6b14729c0fa,
            ])
        ),
    ];

    /// TODO
    #[inline(always)]
    fn mul_fp_by_nonresidue(x: &Self::Fp) -> Self::Fp {
        // times 9
        let mut y = x.clone();
        y.double_in_place(); // 2x
        y += x; // 3x
        y.double_in_place(); // 6x
        y += x; // 7x
        y.double_in_place(); // 8x
        y += x; // 9x
        y
    }
}
