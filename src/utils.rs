use num_bigint::BigUint;

// Utility function that generates round constants
pub fn generate_round_constants(t: usize, rounds: usize, _p: &BigUint) -> Vec<Vec<BigUint>> {
    vec![vec![BigUint::one(); t]; rounds]
}

// Utility function that generates MDS matrix
pub fn generate_mds_matrix(t: usize, _p: &BigUint) -> Vec<Vec<BigUint>> {
    vec![vec![BigUint::one(); t]; t]
}
