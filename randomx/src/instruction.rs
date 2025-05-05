/// The VM executes programs in a special instruction set,
/// which was designed in such way that any random 8-byte word is a valid instruction and any sequence of valid instructions is a valid program.
/// Because there are no "syntax" rules, generating a random program is as easy as filling the program buffer with random data.
///
/// https://github.com/tevador/RandomX/blob/master/doc/specs.md#5-instruction-set
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Instruction {
    // Integer instructions
    IADD_RS,
    IADD_M,
    ISUB_R,
    ISUB_M,
    IMUL_R,
    IMUL_M,
    IMULH_R,
    IMULH_M,
    ISMULH_R,
    ISMULH_M,
    IMUL_RCP,
    INEG_R,
    IXOR_M,
    IROR_R,
    IROL_R,
    ISWAP_R,
    // Floating point instructions
    FSWAP_R,
    FADD_R,
    FADD_M,
    FSUB_R,
    FSUB_M,
    FSCAL_R,
    FMUL_R,
    FDIV_M,
    FSQRT_R,
    // Control instructions
    CFROUND,
    CBRANCH,
    // Store instruction
    ISTORE,
}
