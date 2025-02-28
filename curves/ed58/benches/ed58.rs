use ark_algebra_bench_templates::*;
use ark_ed58::{fq::Fq, fq3::Fq3, fq6::Fq6, fr::Fr, Ed58, G1Projective as G1, G2Projective as G2};

bench!(
    Name = "Ed58",
    Pairing = Ed58,
    G1 = G1,
    G2 = G2,
    ScalarField = Fr,
    G1BaseField = Fq,
    G2BaseField = Fq3,
    TargetField = Fq6,
);
