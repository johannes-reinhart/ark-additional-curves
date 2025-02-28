use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq6 = Fp6<Fq6Config>;

#[derive(Clone, Copy)]
pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp2Config = Fq2Config;

    /// NONRESIDUE = U+2
    const NONRESIDUE: Fq2 = Fq2::new(MontFp!("2"), Fq::ONE);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
        Fq2::new(
            MontFp!(
                "2514746150702782023387381073984499065469793711673933027"
            ),
            MontFp!(
                "3170995372818140140778075409082321578161037698420449167"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
        Fq2::new(
            MontFp!(
                "6804759748846192230227353928539493912877092911468838216"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
        Fq2::new(
            MontFp!("4061635512342874448491275083422895020036595348657332181"),
            MontFp!("3678589620058485595545610840157249645173639502032583691"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
        Fq2::new(
            MontFp!("163175603228860080132500521329690786398206"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
        Fq2::new(
            MontFp!(
                "228378085800698933951926631212232327892033541923971215"
            ),
            MontFp!("6759934504816085075337479327999681603462168004057439988"),
        ),
    ];

    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
        Fq2::new(
            MontFp!("172887046704635624765638747223295244684223803712619877"),
            MontFp!(
                "6648952426623958456964903560021807437046548527634737312"
            ),
        ),
        // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
        Fq2::new(
            MontFp!("163175603228860080132500521329690786398206"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
        Fq2::new(
            MontFp!("2216013510897478354990509493375787861242414648470426925"),
            MontFp!(
                "6792105366014048814374662448682661740983700703914009602"
            ),
        ),
        // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
        Fq2::new(
            MontFp!(
                "6804759748846192230227353928539493912877092911468838216"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
        Fq2::new(
            MontFp!(
                "4415859191244241426074434548020543307471784150072189621"
            ),
            MontFp!("168461705054703540321599568534783648766595972961725932"),
        ),
    ];

    #[inline(always)]
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fq2) -> &mut Fq2 {
        // (c0+u*c1)*(2+u) = (2*c0-c1)+u*(2*c1+c0)
        let mut f = *fe;
        f.double_in_place();
        let mut c0 = fe.c1;
        Fq2Config::mul_fp_by_nonresidue_in_place(&mut c0);
        c0 += &f.c0;
        let c1 = f.c1 + fe.c0;
        *fe = Fq2::new(c0, c1);
        fe
    }
}
