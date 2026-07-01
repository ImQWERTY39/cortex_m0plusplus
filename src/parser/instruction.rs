use super::Immediate;
use super::operand::Address;
use super::operand::RegOrImm;
use super::{Register, RegisterList, SpecialRegister};

pub type WriteBack = bool;
pub type Label = String;

#[derive(Debug)]
pub enum Instruction {
    ADCS(Register, Register, Register),
    ADD(Register, Register, RegOrImm),
    ADDS(Register, Register, RegOrImm),
    MULS(Register, Register), // MULS.0 = MULS.0 * MULS.1
    SUB(Register, Register, RegOrImm),
    SUBS(Register, Register, RegOrImm),
    RSBS(Register, Register, Register),
    SBCS(Register, Register, Register),

    ANDS(Register, Register, Register),
    EORS(Register, Register, Register),
    BICS(Register, Register, Register),
    ORRS(Register, Register, Register),
    TST(Register, Register),

    ASRS(Register, Register, RegOrImm),
    LSLS(Register, Register, RegOrImm),
    LSRS(Register, Register, RegOrImm),
    RORS(Register, Register, Register),

    ADR(Register, Label),
    LDMIA(Register, RegisterList, WriteBack),
    LDR(Register, Address),
    LDRB(Register, Address),
    LDRH(Register, Address),
    LDRSB(Register, Address),
    LDRSH(Register, Address),
    STMIA(Register, RegisterList, WriteBack),
    STR(Register, Address),
    STRB(Register, Address),
    STRH(Register, Address),

    POP(RegisterList),
    PUSH(RegisterList),

    CMP(Register, RegOrImm),
    CMN(Register, RegOrImm),

    B(Label),
    BCC(Label),
    BCS(Label),
    BEQ(Label),
    BGE(Label),
    BGT(Label),
    BHI(Label),
    BHS(Label),
    BL(Label),
    BLE(Label),
    BLO(Label),
    BLS(Label),
    BLT(Label),
    BMI(Label),
    BNE(Label),
    BPL(Label),
    BVC(Label),
    BVS(Label),
    BLX(Register),
    BX(Register),

    MOV(Register, RegOrImm),
    MOVS(Register, RegOrImm),
    MVNS(Register, Register),
    MRS(Register, SpecialRegister),
    MSR(SpecialRegister, Register),

    REV(Register, Register),
    REV16(Register, Register),
    REVSH(Register, Register),

    SXTB(Register, Register),
    SXTH(Register, Register),
    UXTB(Register, Register),
    UXTH(Register, Register),

    SVC(Immediate),
    BKPT(Immediate),
    CPSID,
    CPSIE,
    DMB,
    DSB,
    ISB,
    NOP,
    SEV,
    WFE,
    WFI,
}
