use num_bigint::BigUint;
use num_traits::{Zero, One};
use crate::utils::{generate_round_constants, generate_mds_matrix};

pub struct PoseidonHash {
    t: usize,
    r: usize,
    r_f: usize,
    r_p: usize,
    p: BigUint,
    round_constants: Vec<Vec<BigUint>>,
    mds_matrix: Vec<Vec<BigUint>>,
    output_length: usize,
}

impl PoseidonHash {
    pub fn new(t: usize, r: usize, r_f: usize, r_p: usize, p: BigUint, output_length: usize) -> Self {
        assert!(r < t, "Rate must be less than total state size");
        
        let round_constants = generate_round_constants(t, r_f + r_p, &p);
        let mds_matrix = generate_mds_matrix(t, &p);

        PoseidonHash {
            t, r, r_f, r_p, p,
            round_constants,
            mds_matrix,
            output_length,
        }
    }

    pub fn hash(&self, preimage: &str) -> String {
        let preimage_bytes = preimage.as_bytes();
        let mut state = vec![BigUint::zero(); self.t];
        
        // Absorption phase
        for chunk in preimage_bytes.chunks(32 * self.r) {
            for (i, field_element) in chunk.chunks(32).enumerate().take(self.r) {
                state[i] ^= BigUint::from_bytes_be(field_element);
            }
            state = self.permutation(state);
        }

        // Squeezing phase
        let mut output = Vec::new();
        while output.len() < self.output_length {
            for i in 0..self.r {
                output.extend_from_slice(&state[i].to_bytes_be());
            }
            if output.len() >= self.output_length {
                break;
            }
            state = self.permutation(state);
        }

        output.truncate(self.output_length);

        // Convert the output to a hexadecimal string
        output.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    fn permutation(&self, mut state: Vec<BigUint>) -> Vec<BigUint> {
        // First half of full rounds
        for r in 0..self.r_f / 2 {
            state = self.add_round_constants(state, r);
            state = self.sbox_full(state);
            state = self.mix(state);
        }

        // Partial rounds
        for r in self.r_f / 2..self.r_f / 2 + self.r_p {
            state = self.add_round_constants(state, r);
            state[0] = self.sbox(&state[0]);
            state = self.mix(state);
        }

        // Second half of full rounds
        for r in self.r_f / 2 + self.r_p..self.r_f + self.r_p {
            state = self.add_round_constants(state, r);
            state = self.sbox_full(state);
            state = self.mix(state);
        }

        state
    }

    fn sbox(&self, x: &BigUint) -> BigUint {
        x.modpow(&BigUint::from(7u32), &self.p)
    }

    fn sbox_full(&self, mut state: Vec<BigUint>) -> Vec<BigUint> {
        for i in 0..self.t {
            state[i] = self.sbox(&state[i]);
        }
        state
    }

    fn add_round_constants(&self, mut state: Vec<BigUint>, round: usize) -> Vec<BigUint> {
        for i in 0..self.t {
            state[i] += &self.round_constants[round][i];
            state[i] %= &self.p;
        }
        state
    }

    fn mix(&self, state: Vec<BigUint>) -> Vec<BigUint> {
        let mut new_state = vec![BigUint::zero(); self.t];
        for i in 0..self.t {
            for j in 0..self.t {
                new_state[i] += &self.mds_matrix[i][j] * &state[j];
                new_state[i] %= &self.p;
            }
        }
        new_state
    }
}
