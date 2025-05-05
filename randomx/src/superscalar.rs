//! [Implements the following](https://github.com/tevador/RandomX/blob/master/doc/specs.md#61-instructions)

use blake2b_simd::{Hash, Params};
use std::fmt::Display;

const GENERATOR_LENGTH: usize = 64;
const LATENCY: usize = 170;

pub struct Blake2Generator {
    index: usize,
    data: [u8; GENERATOR_LENGTH],
    gen_params: Params,
}

/// The body of SuperscalarHash is a random sequence of instructions that can run on the Virtual Machine.
/// SuperscalarHash uses a reduced set of only integer register-register instructions listed in Table 6.1.1.
/// dst refers to the destination register, src to the source register.
///
/// https://github.com/tevador/RandomX/blob/master/doc/specs.md#61-instructions
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Instructions {
    ISUB_R,
    IXOR_R,
    IADD_RS,
    IMUL_R,
    IROR_C,
    IADD_C,
    IXOR_C,
    IMULH_R,
    ISMULH_R,
    IMUL_RCP,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum MacroOperation {
    SUB_RR,
    XOR_RR,
    LEA_SIB,
    IMUL_RR,
    ROR_RI,
    ADD_RI,
    XOR_RI,
    MOV_RR,
    MUL_R,
    IMUL_R,
    MOV_RI,
}

#[derive(Debug)]
enum Port {
    /// Port 0, or Port 1, or Port 5.
    P015,
    /// Port 0, or Port 1.
    P01,
    /// Port 0, or Port 5.
    P05,
    /// Port 1
    P1,
    /// Port 5
    P5,
}

/// The size of instructions in the decode stage.
/// Each number is a instruction size, and the sum is 16 for each varient.
#[derive(Debug)]
enum Decoder {
    D3310,
    D3733,
    D4444,
    D484,
    D493,
    D7333,
}

impl Blake2Generator {
    pub fn new(seed: &[u8], nonce: u32) -> Blake2Generator {
        debug_assert!(seed.len() <= GENERATOR_LENGTH - 4);
        let mut params = Params::new();
        params.hash_length(GENERATOR_LENGTH);

        let mut key: [u8; 60] = [0; 60];
        key[..seed.len()].copy_from_slice(seed);

        let mut data: [u8; GENERATOR_LENGTH] = [0; GENERATOR_LENGTH];
        data[..GENERATOR_LENGTH - 4].copy_from_slice(&key);
        data[GENERATOR_LENGTH - 4..GENERATOR_LENGTH].copy_from_slice(&nonce.to_le_bytes());

        Blake2Generator {
            index: GENERATOR_LENGTH,
            data,
            gen_params: params,
        }
    }

    pub fn get_byte(&mut self) -> u8 {
        self.check_data(1);
        let v = self.data[self.index];
        self.index += 1;
        v
    }

    pub fn get_u32(&mut self) -> u32 {
        self.check_data(4);
        let v = u32::from_le_bytes(self.data[self.index..(self.index + 4)].try_into().unwrap());
        self.index += 4;
        v
    }
    fn check_data(&mut self, needed: usize) {
        if self.index + needed > GENERATOR_LENGTH {
            let out = self.gen_params.hash(&self.data);
            self.data = *out.as_array();
            self.index = 0;
        }
    }
}

impl Instructions {
    fn is_multiplication(&self) -> bool {
        match self {
            Instructions::IMUL_R
            | Instructions::IMULH_R
            | Instructions::ISMULH_R
            | Instructions::IMUL_RCP => true,

            Instructions::ISUB_R
            | Instructions::IXOR_R
            | Instructions::IADD_RS
            | Instructions::IROR_C
            | Instructions::IADD_C
            | Instructions::IXOR_C => false,
        }
    }
}

impl MacroOperation {
    pub fn latency(&self) -> usize {
        match self {
            MacroOperation::MOV_RR => 0,

            MacroOperation::SUB_RR
            | MacroOperation::XOR_RR
            | MacroOperation::LEA_SIB
            | MacroOperation::ROR_RI
            | MacroOperation::ADD_RI
            | MacroOperation::XOR_RI
            | MacroOperation::MOV_RI => 1,

            MacroOperation::IMUL_RR => 3,

            MacroOperation::MUL_R | MacroOperation::IMUL_R => 4,
        }
    }

    pub fn port(&self) -> (Option<Port>, Option<Port>) {
        let first = match self {
            MacroOperation::SUB_RR
            | MacroOperation::XOR_RR
            | MacroOperation::ADD_RI
            | MacroOperation::XOR_RI
            | MacroOperation::MOV_RI => Some(Port::P015),

            MacroOperation::LEA_SIB => Some(Port::P01),

            MacroOperation::IMUL_RR | MacroOperation::MUL_R | MacroOperation::IMUL_R => {
                Some(Port::P1)
            }

            MacroOperation::ROR_RI => Some(Port::P05),

            MacroOperation::MOV_RR => None,
        };

        let second = match self {
            MacroOperation::SUB_RR
            | MacroOperation::XOR_RR
            | MacroOperation::LEA_SIB
            | MacroOperation::IMUL_RR
            | MacroOperation::ROR_RI
            | MacroOperation::ADD_RI
            | MacroOperation::XOR_RI
            | MacroOperation::MOV_RR
            | MacroOperation::MOV_RI => None,

            MacroOperation::MUL_R | MacroOperation::IMUL_R => Some(Port::P5),
        };

        (first, second)
    }

    pub fn size(&self) -> usize {
        match self {
            MacroOperation::SUB_RR
            | MacroOperation::XOR_RR
            | MacroOperation::MOV_RR
            | MacroOperation::MUL_R
            | MacroOperation::IMUL_R => 3,

            MacroOperation::LEA_SIB | MacroOperation::IMUL_RR | MacroOperation::ROR_RI => 4,

            // TODO research optimization of padding with nop
            MacroOperation::ADD_RI | MacroOperation::XOR_RI => 7,

            MacroOperation::MOV_RI => 10,
        }
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instructions::ISUB_R => "ISUB_R",
                Instructions::IXOR_R => "IXOR_R",
                Instructions::IADD_RS => "IADD_RS",
                Instructions::IMUL_R => "IMUL_R",
                Instructions::IROR_C => "IROR_C",
                Instructions::IADD_C => "IADD_C",
                Instructions::IXOR_C => "IXOR_C",
                Instructions::IMULH_R => "IMUL_R",
                Instructions::ISMULH_R => "ISMULH_R",
                Instructions::IMUL_RCP => "IMUL_RCP",
            }
        )
    }
}

impl Display for MacroOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MacroOperation::SUB_RR => "sub r, r",
                MacroOperation::XOR_RR => "xor r, r",
                MacroOperation::LEA_SIB => "lea r, r + r * s",
                MacroOperation::IMUL_RR => "imul r, r",
                MacroOperation::ROR_RI => "ror r, i",
                MacroOperation::ADD_RI => "add r, i",
                MacroOperation::XOR_RI => "xor r, i",
                MacroOperation::MOV_RR => "mov r, r",
                MacroOperation::MUL_R => "mul r",
                MacroOperation::IMUL_R => "imul r",
                MacroOperation::MOV_RI => "mov rax, i64",
            }
        )
    }
}
