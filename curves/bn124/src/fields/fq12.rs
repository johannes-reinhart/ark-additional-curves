use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq12 = Fp12<Fq12Config>;

#[derive(Clone, Copy)]
pub struct Fq12Config;

impl Fp12Config for Fq12Config {
    type Fp6Config = Fq6Config;

    const NONRESIDUE: Fq6 = Fq6::new(Fq2::ZERO, Fq2::ONE, Fq2::ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        Fq2::new(
            MontFp!("6060884745197156790398582094782272414"),
            MontFp!(
                "7672369959315728193421712955671707127"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        Fq2::new(
            MontFp!(
                "17000133314539031286158302362289896777"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        Fq2::new(
            MontFp!(
                "6950338899314685341979250055353674380"
            ),
            MontFp!("15804448772966015316831826488521416905"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        Fq2::new(
            MontFp!(
                "17000133314539031286158302362289896776"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        Fq2::new(
            MontFp!("889454154117528551580667960571401966"),
            MontFp!("8132078813650287123410113532849709778"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        Fq2::new(MontFp!("-1"), Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        Fq2::new(
            MontFp!(
                "10939248579595675272620437634320230825"
            ),
            MontFp!("9327763365477103869597306773430796112"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        Fq2::new(
            MontFp!("10253800776860717366812606462"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        Fq2::new(
            MontFp!(
                "10049794425478146721039769673748828859"
            ),
            MontFp!(
                "1195684551826816746187193240581086334"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        Fq2::new(
            MontFp!("10253800776860717366812606463"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        Fq2::new(
            MontFp!(
                "16110679170675303511438351768531101273"
            ),
            MontFp!(
                "8868054511142544939608906196252793461"
            ),
        ),
    ];
}
