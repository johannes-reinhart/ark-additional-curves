use crate::*;
use ark_algebra_test_templates::*;

test_group!(g1; G1Projective; te);
test_group!(g2; G1Projective; te);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<Ed181>; msm);
test_pairing!(pairing; crate::Ed181);