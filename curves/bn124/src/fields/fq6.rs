use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq6 = Fp6<Fq6Config>;

#[derive(Clone, Copy)]
pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp2Config = Fq2Config;

    /// NONRESIDUE = U+5
    const NONRESIDUE: Fq2 = Fq2::new(MontFp!("5"), Fq::ONE);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
        Fq2::new(
            MontFp!(
                "11057421525851115170548223099100677627"
            ),
            MontFp!(
                "5501413929240230020448532534432497469"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
        Fq2::new(
            MontFp!(
                "17000133314539031286158302362289896776"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
        Fq2::new(
            MontFp!("8440916522327550324300911988325232958"),
            MontFp!("16037873359284321335114966383032657210"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
        Fq2::new(
            MontFp!("10253800776860717366812606462"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
        Fq2::new(
            MontFp!(
                "14501928601406998631188904370779095893"
            ),
            MontFp!("12460979361061112770474540540739851799"),
        ),
    ];

    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
        Fq2::new(
            MontFp!("4670603572092301810218237735257106308"),
            MontFp!(
                "14304754188866124854678266550437413591"
            ),
        ),
        // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
        Fq2::new(
            MontFp!("10253800776860717366812606462"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
        Fq2::new(
            MontFp!("4238607547776512410928780877149578148"),
            MontFp!(
                "12026461811321963831273330556256886399"
            ),
        ),
        // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
        Fq2::new(
            MontFp!(
                "17000133314539031286158302362289896776"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
        Fq2::new(
            MontFp!(
                "8090922204924017841872001116695818783"
            ),
            MontFp!("7669050649397575440086442351510706488"),
        ),
    ];

    #[inline(always)]
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fq2) -> &mut Fq2 {
        // (c0+u*c1)*(5+u) = (5*c0-c1)+u*(5*c1+c0)
        let mut f = *fe;
        f.double_in_place().double_in_place();
        let mut c0 = fe.c1;
        Fq2Config::mul_fp_by_nonresidue_in_place(&mut c0);
        c0 += &f.c0;
        c0 += &fe.c0;
        let c1 = f.c1 + fe.c1 + fe.c0;
        *fe = Fq2::new(c0, c1);
        fe
    }
}
