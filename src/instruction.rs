#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Mnemonic {
    NOP,
    DAA,
    CPL,
    SCF,
    CCF,
    RST(u16),
    JP_nn,
    JP_c_nn(Flag),
    JP_nc_nn(Flag),
    JP_hl,
    JR_c_e(Flag),
    JR_nc_e(Flag),
    JR_e,
    CP_n,
    CP_r(Target),
    CP_hl,
    CALL_nn,
    CALL_c_nn(Flag),
    CALL_nc_nn(Flag),
    AND_r(Target),
    AND_n,
    AND_hl,
    ADD_r(Target),
    ADD_a_hl,
    ADD_hl_rr(Target),
    ADD_hl_sp,
    ADD_sp_n,
    ADD_n,
    ADC_r(Target),
    ADC_n,
    ADC_hl,
    INC_r(Target),
    INC_rr(Target),
    INC_hl,
    INC_sp,
    DEC_r(Target),
    DEC_rr(Target),
    DEC_sp,
    DEC_hl,
    POP_rr(Target),
    POP_af,
    SUB_r(Target),
    SUB_n,
    SUB_hl,
    SBC_r(Target),
    SBC_hl,
    OR_r(Target),
    OR_n,
    OR_hl,
    XOR_r(Target),
    XOR_n,
    XOR_hl,
    LD_r_r(Target, Target),
    LD_r_n(Target),
    LD_a_nn,
    LD_hl_n,
    LD_hl_plus_a,
    LD_hl_minus_a,
    LD_a_hl_plus,
    LD_a_hl_minus,
    LD_hl_sp_plus_n,
    LD_rr_r(Target, Target),
    LD_rr_nn(Target),
    LD_r_rr(Target, Target),
    LD_nn_a,
    LDH_n_a,
    LDH_a_n,
    LD_sp_nn,
    LD_sp_hl,
    LD_nn_sp,
    PUSH_rr(Target),
    DisableInterrupt,
    RRCA,
    RRA,
    RLCA,
    RLA,
    RLC_r(Target),
    RRC_r(Target),
    RL_r(Target),
    RET,
    RETI,
    RET_c(Flag),
    RET_nc(Flag),
    Prefix,
    RES_b_r(u8, Target),
    SRL_r(Target),
    SLA_r(Target),
    SRA_r(Target),
    RR_r(Target),
    SWAP_r(Target),
}

#[derive(Debug)]
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub length: u16,
    m_cycles: u8,
}

impl Instruction {
    fn new(mnemonic: Mnemonic, length: u16, m_cycles: u8) -> Self {
        Self {
            mnemonic,
            length,
            m_cycles,
        }
    }

    pub fn from_byte(value: u8) -> Self {
        match value {
            0x00 => Instruction::new(Mnemonic::NOP, 1, 1),
            0x01 => Instruction::new(Mnemonic::LD_rr_nn(Target::BC), 3, 3),
            0x02 => Instruction::new(Mnemonic::LD_rr_r(Target::BC, Target::A), 1, 2),
            0x03 => Instruction::new(Mnemonic::INC_rr(Target::BC), 1, 2),
            0x04 => Instruction::new(Mnemonic::INC_r(Target::B), 1, 1),
            0x05 => Instruction::new(Mnemonic::DEC_r(Target::B), 1, 1),
            0x06 => Instruction::new(Mnemonic::LD_r_n(Target::B), 2, 2),
            0x07 => Instruction::new(Mnemonic::RLCA, 1, 1),
            0x08 => Instruction::new(Mnemonic::LD_nn_sp, 3, 5),
            0x0A => Instruction::new(Mnemonic::LD_r_rr(Target::A, Target::BC), 1, 2),
            0x0B => Instruction::new(Mnemonic::DEC_rr(Target::BC), 1, 2),
            0x0C => Instruction::new(Mnemonic::INC_r(Target::C), 1, 1),
            0x0D => Instruction::new(Mnemonic::DEC_r(Target::C), 1, 1),
            0x0E => Instruction::new(Mnemonic::LD_r_n(Target::C), 2, 2),
            0x0F => Instruction::new(Mnemonic::RRCA, 1, 1),
            0x11 => Instruction::new(Mnemonic::LD_rr_nn(Target::DE), 3, 3),
            0x12 => Instruction::new(Mnemonic::LD_rr_r(Target::DE, Target::A), 1, 2),
            0x13 => Instruction::new(Mnemonic::INC_rr(Target::DE), 1, 2),
            0x14 => Instruction::new(Mnemonic::INC_r(Target::D), 1, 1),
            0x15 => Instruction::new(Mnemonic::DEC_r(Target::D), 1, 1),
            0x16 => Instruction::new(Mnemonic::LD_r_n(Target::D), 2, 2),
            0x17 => Instruction::new(Mnemonic::RLA, 1, 1),
            0x18 => Instruction::new(Mnemonic::JR_e, 2, 3),
            0x1A => Instruction::new(Mnemonic::LD_r_rr(Target::A, Target::DE), 1, 2),
            0x1D => Instruction::new(Mnemonic::DEC_r(Target::E), 1, 1),
            0x1C => Instruction::new(Mnemonic::INC_r(Target::E), 1, 1),
            0x1E => Instruction::new(Mnemonic::LD_r_n(Target::E), 2, 2),
            0x1F => Instruction::new(Mnemonic::RRA, 1, 1),
            0x20 => Instruction::new(Mnemonic::JR_nc_e(Flag::Z), 2, 3),
            0x21 => Instruction::new(Mnemonic::LD_rr_nn(Target::HL), 3, 3),
            0x22 => Instruction::new(Mnemonic::LD_hl_plus_a, 1, 2),
            0x23 => Instruction::new(Mnemonic::INC_rr(Target::HL), 1, 2),
            0x24 => Instruction::new(Mnemonic::INC_r(Target::H), 1, 1),
            0x25 => Instruction::new(Mnemonic::DEC_r(Target::H), 1, 1),
            0x26 => Instruction::new(Mnemonic::LD_r_n(Target::H), 2, 2),
            0x27 => Instruction::new(Mnemonic::DAA, 1, 1),
            0x28 => Instruction::new(Mnemonic::JR_c_e(Flag::Z), 2, 2),
            0x29 => Instruction::new(Mnemonic::ADD_hl_rr(Target::HL), 1, 2),
            0x2A => Instruction::new(Mnemonic::LD_a_hl_plus, 1, 2),
            0x2C => Instruction::new(Mnemonic::INC_r(Target::L), 1, 1),
            0x2D => Instruction::new(Mnemonic::DEC_r(Target::L), 1, 1),
            0x2E => Instruction::new(Mnemonic::LD_r_n(Target::L), 2, 2),
            0x2F => Instruction::new(Mnemonic::CPL, 1, 1),
            0x30 => Instruction::new(Mnemonic::JR_nc_e(Flag::C), 2, 3),
            0x31 => Instruction::new(Mnemonic::LD_sp_nn, 3, 3),
            0x32 => Instruction::new(Mnemonic::LD_hl_minus_a, 1, 2),
            0x33 => Instruction::new(Mnemonic::INC_sp, 1, 2),
            0x34 => Instruction::new(Mnemonic::INC_hl, 1, 3),
            0x35 => Instruction::new(Mnemonic::DEC_hl, 1, 3),
            0x36 => Instruction::new(Mnemonic::LD_hl_n, 2, 3),
            0x37 => Instruction::new(Mnemonic::SCF, 1, 1),
            0x38 => Instruction::new(Mnemonic::JR_c_e(Flag::C), 2, 2),
            0x39 => Instruction::new(Mnemonic::ADD_hl_sp, 1, 2),
            0x3A => Instruction::new(Mnemonic::LD_a_hl_minus, 1, 2),
            0x3B => Instruction::new(Mnemonic::DEC_sp, 1, 2),
            0x3C => Instruction::new(Mnemonic::INC_r(Target::A), 1, 1),
            0x3D => Instruction::new(Mnemonic::DEC_r(Target::A), 1, 1),
            0x3E => Instruction::new(Mnemonic::LD_r_n(Target::A), 2, 2),
            0x3F => Instruction::new(Mnemonic::CCF, 1, 1),
            0x40 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::B), 1, 1),
            0x41 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::C), 1, 1),
            0x42 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::D), 1, 1),
            0x43 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::E), 1, 1),
            0x44 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::H), 1, 1),
            0x45 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::L), 1, 1),
            0x46 => Instruction::new(Mnemonic::LD_r_rr(Target::B, Target::HL), 1, 2),
            0x47 => Instruction::new(Mnemonic::LD_r_r(Target::B, Target::A), 1, 1),
            0x48 => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::B), 1, 1),
            0x49 => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::C), 1, 1),
            0x4A => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::D), 1, 1),
            0x4B => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::E), 1, 1),
            0x4C => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::H), 1, 1),
            0x4D => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::L), 1, 1),
            0x4E => Instruction::new(Mnemonic::LD_r_rr(Target::C, Target::HL), 1, 2),
            0x4F => Instruction::new(Mnemonic::LD_r_r(Target::C, Target::A), 1, 1),
            0x50 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::B), 1, 1),
            0x51 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::C), 1, 1),
            0x52 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::D), 1, 1),
            0x53 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::E), 1, 1),
            0x54 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::H), 1, 1),
            0x55 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::L), 1, 1),
            0x56 => Instruction::new(Mnemonic::LD_r_rr(Target::D, Target::HL), 1, 2),
            0x57 => Instruction::new(Mnemonic::LD_r_r(Target::D, Target::A), 1, 1),
            0x58 => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::B), 1, 1),
            0x59 => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::C), 1, 1),
            0x5A => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::D), 1, 1),
            0x5B => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::E), 1, 1),
            0x5C => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::H), 1, 1),
            0x5D => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::L), 1, 1),
            0x5E => Instruction::new(Mnemonic::LD_r_rr(Target::E, Target::HL), 1, 2),
            0x5F => Instruction::new(Mnemonic::LD_r_r(Target::E, Target::A), 1, 1),
            0x60 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::B), 1, 1),
            0x61 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::C), 1, 1),
            0x62 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::D), 1, 1),
            0x63 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::E), 1, 1),
            0x64 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::H), 1, 1),
            0x65 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::L), 1, 1),
            0x66 => Instruction::new(Mnemonic::LD_r_rr(Target::H, Target::HL), 1, 2),
            0x67 => Instruction::new(Mnemonic::LD_r_r(Target::H, Target::A), 1, 1),
            0x68 => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::B), 1, 1),
            0x69 => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::C), 1, 1),
            0x6A => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::D), 1, 1),
            0x6B => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::E), 1, 1),
            0x6C => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::H), 1, 1),
            0x6D => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::L), 1, 1),
            0x6E => Instruction::new(Mnemonic::LD_r_rr(Target::L, Target::HL), 1, 2),
            0x6F => Instruction::new(Mnemonic::LD_r_r(Target::L, Target::A), 1, 1),
            0x70 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::B), 1, 2),
            0x71 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::C), 1, 2),
            0x72 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::D), 1, 2),
            0x73 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::E), 1, 2),
            0x74 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::H), 1, 2),
            0x75 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::L), 1, 2),
            0x77 => Instruction::new(Mnemonic::LD_rr_r(Target::HL, Target::A), 1, 2),
            0x78 => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::B), 1, 1),
            0x79 => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::C), 1, 1),
            0x7A => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::D), 1, 1),
            0x7B => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::E), 1, 1),
            0x7C => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::H), 1, 1),
            0x7D => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::L), 1, 1),
            0x7E => Instruction::new(Mnemonic::LD_r_rr(Target::A, Target::HL), 1, 2),
            0x7F => Instruction::new(Mnemonic::LD_r_r(Target::A, Target::A), 1, 1),
            0x80 => Instruction::new(Mnemonic::ADD_r(Target::B), 1, 1),
            0x81 => Instruction::new(Mnemonic::ADD_r(Target::C), 1, 1),
            0x82 => Instruction::new(Mnemonic::ADD_r(Target::D), 1, 1),
            0x83 => Instruction::new(Mnemonic::ADD_r(Target::E), 1, 1),
            0x84 => Instruction::new(Mnemonic::ADD_r(Target::H), 1, 1),
            0x85 => Instruction::new(Mnemonic::ADD_r(Target::L), 1, 1),
            0x86 => Instruction::new(Mnemonic::ADD_a_hl, 1, 2),
            0x87 => Instruction::new(Mnemonic::ADD_r(Target::A), 1, 1),
            0x88 => Instruction::new(Mnemonic::ADC_r(Target::B), 1, 1),
            0x89 => Instruction::new(Mnemonic::ADC_r(Target::C), 1, 1),
            0x8A => Instruction::new(Mnemonic::ADC_r(Target::D), 1, 1),
            0x8B => Instruction::new(Mnemonic::ADC_r(Target::E), 1, 1),
            0x8C => Instruction::new(Mnemonic::ADC_r(Target::H), 1, 1),
            0x8D => Instruction::new(Mnemonic::ADC_r(Target::L), 1, 1),
            0x8E => Instruction::new(Mnemonic::ADC_hl, 1, 2),
            0x8F => Instruction::new(Mnemonic::ADC_r(Target::A), 1, 1),
            0x90 => Instruction::new(Mnemonic::SUB_r(Target::B), 1, 1),
            0x91 => Instruction::new(Mnemonic::SUB_r(Target::C), 1, 1),
            0x92 => Instruction::new(Mnemonic::SUB_r(Target::D), 1, 1),
            0x93 => Instruction::new(Mnemonic::SUB_r(Target::E), 1, 1),
            0x94 => Instruction::new(Mnemonic::SUB_r(Target::H), 1, 1),
            0x95 => Instruction::new(Mnemonic::SUB_r(Target::L), 1, 1),
            0x96 => Instruction::new(Mnemonic::SUB_hl, 1, 2),
            0x97 => Instruction::new(Mnemonic::SUB_r(Target::A), 1, 1),
            0x98 => Instruction::new(Mnemonic::SBC_r(Target::B), 1, 1),
            0x99 => Instruction::new(Mnemonic::SBC_r(Target::C), 1, 1),
            0x9A => Instruction::new(Mnemonic::SBC_r(Target::D), 1, 1),
            0x9B => Instruction::new(Mnemonic::SBC_r(Target::E), 1, 1),
            0x9C => Instruction::new(Mnemonic::SBC_r(Target::H), 1, 1),
            0x9D => Instruction::new(Mnemonic::SBC_r(Target::L), 1, 1),
            0x9E => Instruction::new(Mnemonic::SBC_hl, 1, 2),
            0x9F => Instruction::new(Mnemonic::SBC_r(Target::A), 1, 1),
            0xA0 => Instruction::new(Mnemonic::AND_r(Target::B), 1, 1),
            0xA1 => Instruction::new(Mnemonic::AND_r(Target::C), 1, 1),
            0xA2 => Instruction::new(Mnemonic::AND_r(Target::D), 1, 1),
            0xA3 => Instruction::new(Mnemonic::AND_r(Target::E), 1, 1),
            0xA4 => Instruction::new(Mnemonic::AND_r(Target::H), 1, 1),
            0xA5 => Instruction::new(Mnemonic::AND_r(Target::L), 1, 1),
            0xA6 => Instruction::new(Mnemonic::AND_hl, 1, 2),
            0xA7 => Instruction::new(Mnemonic::AND_r(Target::A), 1, 1),
            0xA8 => Instruction::new(Mnemonic::XOR_r(Target::B), 1, 1),
            0xA9 => Instruction::new(Mnemonic::XOR_r(Target::C), 1, 1),
            0xAA => Instruction::new(Mnemonic::XOR_r(Target::D), 1, 1),
            0xAB => Instruction::new(Mnemonic::XOR_r(Target::E), 1, 1),
            0xAC => Instruction::new(Mnemonic::XOR_r(Target::H), 1, 1),
            0xAD => Instruction::new(Mnemonic::XOR_r(Target::L), 1, 1),
            0xAE => Instruction::new(Mnemonic::XOR_hl, 1, 2),
            0xAF => Instruction::new(Mnemonic::XOR_r(Target::A), 1, 1),
            0xB0 => Instruction::new(Mnemonic::OR_r(Target::B), 1, 1),
            0xB1 => Instruction::new(Mnemonic::OR_r(Target::C), 1, 1),
            0xB2 => Instruction::new(Mnemonic::OR_r(Target::D), 1, 1),
            0xB3 => Instruction::new(Mnemonic::OR_r(Target::E), 1, 1),
            0xB4 => Instruction::new(Mnemonic::OR_r(Target::H), 1, 1),
            0xB5 => Instruction::new(Mnemonic::OR_r(Target::L), 1, 1),
            0xB6 => Instruction::new(Mnemonic::OR_hl, 1, 2),
            0xB7 => Instruction::new(Mnemonic::OR_r(Target::A), 1, 1),
            0xB8 => Instruction::new(Mnemonic::CP_r(Target::B), 1, 1),
            0xB9 => Instruction::new(Mnemonic::CP_r(Target::C), 1, 1),
            0xBA => Instruction::new(Mnemonic::CP_r(Target::D), 1, 1),
            0xBB => Instruction::new(Mnemonic::CP_r(Target::E), 1, 1),
            0xBC => Instruction::new(Mnemonic::CP_r(Target::H), 1, 1),
            0xBD => Instruction::new(Mnemonic::CP_r(Target::L), 1, 1),
            0xBE => Instruction::new(Mnemonic::CP_hl, 1, 2),
            0xBF => Instruction::new(Mnemonic::CP_r(Target::A), 1, 1),
            0xC0 => Instruction::new(Mnemonic::RET_nc(Flag::Z), 1, 2),
            0xC1 => Instruction::new(Mnemonic::POP_rr(Target::BC), 1, 3),
            0xC2 => Instruction::new(Mnemonic::JP_nc_nn(Flag::Z), 3, 3),
            0xC3 => Instruction::new(Mnemonic::JP_nn, 3, 4),
            0xC4 => Instruction::new(Mnemonic::CALL_nc_nn(Flag::Z), 3, 3),
            0xC5 => Instruction::new(Mnemonic::PUSH_rr(Target::BC), 1, 4),
            0xC6 => Instruction::new(Mnemonic::ADD_n, 2, 2),
            0xC7 => Instruction::new(Mnemonic::RST(0x0000), 1, 4),
            0xC8 => Instruction::new(Mnemonic::RET_c(Flag::Z), 1, 2),
            0xC9 => Instruction::new(Mnemonic::RET, 1, 4),
            0xCA => Instruction::new(Mnemonic::JP_c_nn(Flag::Z), 3, 3),
            0xCB => Instruction::new(Mnemonic::Prefix, 1, 1),
            0xCC => Instruction::new(Mnemonic::CALL_c_nn(Flag::Z), 3, 3),
            0xCD => Instruction::new(Mnemonic::CALL_nn, 3, 6),
            0xCE => Instruction::new(Mnemonic::ADC_n, 2, 2),
            0xCF => Instruction::new(Mnemonic::RST(0x0008), 1, 4),
            0xD0 => Instruction::new(Mnemonic::RET_nc(Flag::C), 1, 2),
            0xD1 => Instruction::new(Mnemonic::POP_rr(Target::DE), 1, 3),
            0xD2 => Instruction::new(Mnemonic::JP_nc_nn(Flag::C), 3, 12),
            0xD4 => Instruction::new(Mnemonic::CALL_nc_nn(Flag::C), 3, 3),
            0xD5 => Instruction::new(Mnemonic::PUSH_rr(Target::DE), 1, 4),
            0xD6 => Instruction::new(Mnemonic::SUB_n, 2, 2),
            0xD7 => Instruction::new(Mnemonic::RST(0x0010), 1, 4),
            0xD8 => Instruction::new(Mnemonic::RET_c(Flag::C), 1, 2),
            0xD9 => Instruction::new(Mnemonic::RETI, 3, 3),
            0xDA => Instruction::new(Mnemonic::JP_c_nn(Flag::C), 3, 3),
            0xDC => Instruction::new(Mnemonic::CALL_c_nn(Flag::C), 3, 3),
            0xDF => Instruction::new(Mnemonic::RST(0x0018), 1, 4),
            0xE0 => Instruction::new(Mnemonic::LDH_n_a, 2, 3),
            0xE1 => Instruction::new(Mnemonic::POP_rr(Target::HL), 1, 3),
            0xE5 => Instruction::new(Mnemonic::PUSH_rr(Target::HL), 1, 4),
            0xE6 => Instruction::new(Mnemonic::AND_n, 2, 2),
            0xE7 => Instruction::new(Mnemonic::RST(0x0020), 1, 4),
            0xE8 => Instruction::new(Mnemonic::ADD_sp_n, 2, 4),
            0xE9 => Instruction::new(Mnemonic::JP_hl, 1, 1),
            0xEA => Instruction::new(Mnemonic::LD_nn_a, 3, 4),
            0xEE => Instruction::new(Mnemonic::XOR_n, 2, 2),
            0xEF => Instruction::new(Mnemonic::RST(0x0028), 1, 4),
            0xF0 => Instruction::new(Mnemonic::LDH_a_n, 2, 3),
            0xF1 => Instruction::new(Mnemonic::POP_af, 1, 3),
            0xF3 => Instruction::new(Mnemonic::DisableInterrupt, 1, 1),
            0xF5 => Instruction::new(Mnemonic::PUSH_rr(Target::AF), 1, 4),
            0xF6 => Instruction::new(Mnemonic::OR_n, 2, 2),
            0xF7 => Instruction::new(Mnemonic::RST(0x0030), 1, 4),
            0xF8 => Instruction::new(Mnemonic::LD_hl_sp_plus_n, 2, 3),
            0xF9 => Instruction::new(Mnemonic::LD_sp_hl, 1, 2),
            0xFA => Instruction::new(Mnemonic::LD_a_nn, 3, 4),
            0xFE => Instruction::new(Mnemonic::CP_n, 2, 2),
            0xFF => Instruction::new(Mnemonic::RST(0x0038), 1, 4),
            _ => panic!("Instruction for byte {:#X} not implemented.", value),
        }
    }

    pub fn from_prefix(value: u8) -> Self {
        match value {
            0x00 => Instruction::new(Mnemonic::RLC_r(Target::B), 2, 2),
            0x01 => Instruction::new(Mnemonic::RLC_r(Target::C), 2, 2),
            0x02 => Instruction::new(Mnemonic::RLC_r(Target::D), 2, 2),
            0x03 => Instruction::new(Mnemonic::RLC_r(Target::E), 2, 2),
            0x04 => Instruction::new(Mnemonic::RLC_r(Target::H), 2, 2),
            0x05 => Instruction::new(Mnemonic::RLC_r(Target::L), 2, 2),
            0x07 => Instruction::new(Mnemonic::RLC_r(Target::A), 2, 2),
            0x08 => Instruction::new(Mnemonic::RRC_r(Target::B), 2, 2),
            0x09 => Instruction::new(Mnemonic::RRC_r(Target::C), 2, 2),
            0x0A => Instruction::new(Mnemonic::RRC_r(Target::D), 2, 2),
            0x0B => Instruction::new(Mnemonic::RRC_r(Target::E), 2, 2),
            0x0C => Instruction::new(Mnemonic::RRC_r(Target::H), 2, 2),
            0x0D => Instruction::new(Mnemonic::RRC_r(Target::L), 2, 2),
            0x0F => Instruction::new(Mnemonic::RRC_r(Target::A), 2, 2),
            0x10 => Instruction::new(Mnemonic::RL_r(Target::B), 2, 2),
            0x11 => Instruction::new(Mnemonic::RL_r(Target::C), 2, 2),
            0x12 => Instruction::new(Mnemonic::RL_r(Target::D), 2, 2),
            0x13 => Instruction::new(Mnemonic::RL_r(Target::E), 2, 2),
            0x14 => Instruction::new(Mnemonic::RL_r(Target::H), 2, 2),
            0x15 => Instruction::new(Mnemonic::RL_r(Target::L), 2, 2),
            0x17 => Instruction::new(Mnemonic::RL_r(Target::A), 2, 2),
            0x18 => Instruction::new(Mnemonic::RR_r(Target::B), 2, 2),
            0x19 => Instruction::new(Mnemonic::RR_r(Target::C), 2, 2),
            0x1A => Instruction::new(Mnemonic::RR_r(Target::D), 2, 2),
            0x1B => Instruction::new(Mnemonic::RR_r(Target::E), 2, 2),
            0x1C => Instruction::new(Mnemonic::RR_r(Target::H), 2, 2),
            0x1D => Instruction::new(Mnemonic::RR_r(Target::L), 2, 2),
            0x1F => Instruction::new(Mnemonic::RR_r(Target::A), 2, 2),
            0x20 => Instruction::new(Mnemonic::SLA_r(Target::B), 2, 2),
            0x21 => Instruction::new(Mnemonic::SLA_r(Target::C), 2, 2),
            0x22 => Instruction::new(Mnemonic::SLA_r(Target::D), 2, 2),
            0x23 => Instruction::new(Mnemonic::SLA_r(Target::E), 2, 2),
            0x24 => Instruction::new(Mnemonic::SLA_r(Target::H), 2, 2),
            0x25 => Instruction::new(Mnemonic::SLA_r(Target::L), 2, 2),
            0x27 => Instruction::new(Mnemonic::SLA_r(Target::A), 2, 2),
            0x28 => Instruction::new(Mnemonic::SRA_r(Target::B), 2, 2),
            0x29 => Instruction::new(Mnemonic::SRA_r(Target::C), 2, 2),
            0x2A => Instruction::new(Mnemonic::SRA_r(Target::D), 2, 2),
            0x2B => Instruction::new(Mnemonic::SRA_r(Target::E), 2, 2),
            0x2C => Instruction::new(Mnemonic::SRA_r(Target::H), 2, 2),
            0x2D => Instruction::new(Mnemonic::SRA_r(Target::L), 2, 2),
            0x2F => Instruction::new(Mnemonic::SRA_r(Target::A), 2, 2),
            0x30 => Instruction::new(Mnemonic::SWAP_r(Target::B), 2, 2),
            0x31 => Instruction::new(Mnemonic::SWAP_r(Target::C), 2, 2),
            0x32 => Instruction::new(Mnemonic::SWAP_r(Target::D), 2, 2),
            0x33 => Instruction::new(Mnemonic::SWAP_r(Target::E), 2, 2),
            0x34 => Instruction::new(Mnemonic::SWAP_r(Target::H), 2, 2),
            0x35 => Instruction::new(Mnemonic::SWAP_r(Target::L), 2, 2),
            0x37 => Instruction::new(Mnemonic::SWAP_r(Target::A), 2, 2),
            0x38 => Instruction::new(Mnemonic::SRL_r(Target::B), 2, 2),
            0x39 => Instruction::new(Mnemonic::SRL_r(Target::C), 2, 2),
            0x3A => Instruction::new(Mnemonic::SRL_r(Target::D), 2, 2),
            0x3B => Instruction::new(Mnemonic::SRL_r(Target::E), 2, 2),
            0x3C => Instruction::new(Mnemonic::SRL_r(Target::H), 2, 2),
            0x3D => Instruction::new(Mnemonic::SRL_r(Target::L), 2, 2),
            0x3F => Instruction::new(Mnemonic::SRL_r(Target::A), 2, 2),
            0x87 => Instruction::new(Mnemonic::RES_b_r(0, Target::A), 2, 2),
            _ => panic!("PREFIX Instruction for byte {:#X} not implemented.", value),
        }
    }
}
