use super::Immediate;
use super::operand::Address;
use super::operand::RegOrImm;
use super::{Register, RegisterList, SpecialRegister};

pub type WriteBack = bool;
pub type Label = String;

#[derive(Debug)]
pub enum Instruction {
    /*
     * <adcs rd=Register rn=Register />
     *     rd and rn must be low registers
     *     Updates flags: NZCV
     *     Opcode: 0100000101<rn:3><rd:3>
     *     1 Clock cycle
     */
    ADCS(Register, Register),

    /*
     * <add rd=Register rn=Register />
     *     Atleast 1 High Register
     *     Opcode: 01000100<dn:1><rm:3><rd:3>
     *
     * <add rd=Register rn=["PC"|"SP"] imm=Immediate />
     *     rd must be a low register
     *     imm is multiplied by 4, word aligned range of 0-1020
     *     Opcode: 10100<rd:3><imm:8> (PC),
     *             10101<rd:3><imm:8> (SP)
     *
     * <add rd="SP" imm=Immediate />
     *     imm is multiplied by 4, word aligned range of 0-508
     *     Opcode: 101100000<imm:7>
     *
     * 1 Clock cycle
     */
    ADD(Register, Register, RegOrImm),

    /*
     * <adds rd=Register rn=Register rm=Register />
     *     all registers must be low registers
     *     Opcode: 0001100<rm:3><rn:3><rd:3>
     *
     * <adds rd=Register rn=Register imm=Immediate/>
     *     all registers must be low registers
     *     imm in the range 0-7
     *     Opcode: 0001100<imm:3><rn:3><rd:3>
     *
     * <adds rd=Register imm=Immediate />
     *     rd must be low register
     *     imm in the range 0-255
     *     Opcode: 00110<rd:3><imm:8>
     *
     * 1 Clock cycle
     * Updates flags: NZCV
     */
    ADDS(Register, Register, RegOrImm),

    /*
     * <muls rd=Register rn=Register />
     *     all registers must be low
     *     Opcode: 0100000111<rn:3><rd:3>
     *     Update flags: NZ
     *     1/32 clock cycles
     */
    MULS(Register, Register), // MULS.0 = MULS.0 * MULS.1

    /*
     * <sub rd=Register imm=Immediatae />
     *     rd must be SP
     *     imm is multiplied by 4, word aligned range of 0-508
     *     1 Clock cycle
     *     Opcode: 101100001<imm:7>
     */
    SUB(Register, Immediate),

    /*
     * <subs rd=Register rn=Register rm=Register />
     *     all registers must be low registers
     *     Opcode: 0001101<rm:3><rn:3><rd:3>
     *
     * <subs rd=Register rn=Register imm=Immediate/>
     *     all registers must be low registers
     *     imm in the range 0-7
     *     Opcode: 0001111<imm:3><rn:3><rd:3>
     *
     * <subs rd=Register imm=Immediate />
     *     rd must be low register
     *     imm in the range 0-255
     *     Opcode: 00111<rd:3><imm:8>
     *
     * 1 Clock cycle
     * Updates flags: NZCV
     */
    SUBS(Register, Register, RegOrImm),

    /*
     * <rsbs rd=Register rn=Register />
     *     all registers must be low registers
     *     Opcode: 0100001001<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZCV
     */
    RSBS(Register, Register),

    /*
     * <sbcs rd=Register rn=Register />
     *     all registers must be low registers
     *     Opcode: 0100000110<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZCV
     */
    SBCS(Register, Register),

    /*
     * <ands rd=Register rn=Register />
     *     all registes must be low registers
     *     Opcode: 0100000000<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZ
     */
    ANDS(Register, Register),

    /*
     * <eors rd=Register rn=Register />
     *     all registes must be low registers
     *     Opcode: 0100000001<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZ
     */
    EORS(Register, Register),

    /*
     * <bics rd=Register rn=Register />
     *     all registes must be low registers
     *     Opcode: 0100001110<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZ
     */
    BICS(Register, Register),

    /*
     * <orrs rd=Register rn=Register />
     *     all registes must be low registers
     *     Opcode: 0100001100<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZ
     */
    ORRS(Register, Register),

    /*
     * <tst rn=Register rm=Register />
     *     all registes must be low registers, result not saved
     *     Opcode: 0100001000<rn:3><rd:3>
     *     1 Clock cycle
     *     Updates flags: NZ
     */
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

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::ADCS(register, register1) => {
                write!(f, "\tADCS\t\t{}, {}", register, register1)
            }
            Instruction::ADD(register, register1, register2) => {
                write!(f, "\tADD\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::ADDS(register, register1, reg_or_imm) => {
                if register == register1 && matches!(reg_or_imm, RegOrImm::Immediate(_)) {
                    write!(f, "\tADDS\t\t{}, {}", register, reg_or_imm)
                } else {
                    write!(f, "\tADDS\t\t{}, {}, {}", register, register1, reg_or_imm)
                }
            }
            Instruction::MULS(register, register1) => {
                write!(f, "\tMULS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::SUB(register, imm) => {
                write!(f, "\tSUB\t\t{}, {}, {}", register, register, imm)
            }
            Instruction::SUBS(register, register1, reg_or_imm) => {
                if register == register1 && matches!(reg_or_imm, RegOrImm::Immediate(_)) {
                    write!(f, "\tSUBS\t\t{}, {}", register, reg_or_imm)
                } else {
                    write!(f, "\tSUBS\t\t{}, {}, {}", register, register1, reg_or_imm)
                }
            }
            Instruction::RSBS(register, register1) => {
                write!(f, "\tRSBS\t\t{}, {}, #0", register, register1)
            }
            Instruction::SBCS(register, register1) => {
                write!(f, "\tSBCS\t\t{}, {}, {}", register, register, register1)
            }

            Instruction::ANDS(register, register1) => {
                write!(f, "\tANDS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::EORS(register, register1) => {
                write!(f, "\tEORS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::BICS(register, register1) => {
                write!(f, "\tBICS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::ORRS(register, register1) => {
                write!(f, "\tORRS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::TST(register, register1) => {
                write!(f, "\tTST\t\t{}, {}", register, register1)
            }
            Instruction::ASRS(register, register1, register2) => {
                write!(f, "\tASRS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::LSLS(register, register1, register2) => {
                write!(f, "\tLSLS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::LSRS(register, register1, register2) => {
                write!(f, "\tLSRS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::RORS(register, register1, register2) => {
                write!(f, "\tRORS\t\t{}, {}, {}", register, register1, register2)
            }

            Instruction::ADR(register, label) => write!(f, "\tADR\t\t{}, {}", register, label),
            Instruction::LDMIA(register, reg_list, write_back) => write!(
                f,
                "\tLDMIA\t\t{}{}, {}",
                register,
                if *write_back { '!' } else { ' ' },
                reg_list
            ),
            Instruction::LDR(reg, addr) => write!(f, "\tLDR\t\t{}, {}", reg, addr),
            Instruction::LDRB(reg, addr) => write!(f, "\tLDRB\t\t{}, {}", reg, addr),
            Instruction::LDRH(reg, addr) => write!(f, "\tLDRH\t\t{}, {}", reg, addr),
            Instruction::LDRSB(reg, addr) => write!(f, "\tLDRSB\t\t{}, {}", reg, addr),
            Instruction::LDRSH(reg, addr) => write!(f, "\tLDRSH\t\t{}, {}", reg, addr),
            Instruction::STMIA(register, reg_list, write_back) => write!(
                f,
                "\tSTMIA\t\t{}{}, {}",
                register,
                if *write_back { '!' } else { ' ' },
                reg_list
            ),
            Instruction::STR(reg, addr) => write!(f, "\tSTR\t\t{}, {}", reg, addr),
            Instruction::STRB(reg, addr) => write!(f, "\tSTRB\t\t{}, {}", reg, addr),
            Instruction::STRH(reg, addr) => write!(f, "\tSTRH\t\t{}, {}", reg, addr),

            Instruction::POP(reg_list) => write!(f, "\tPOP\t\t{}", reg_list),
            Instruction::PUSH(reg_list) => write!(f, "\tPUSH\t\t{}", reg_list),

            Instruction::CMP(reg, reg_or_imm) => write!(f, "\tCMP\t\t{}, {}", reg, reg_or_imm),
            Instruction::CMN(reg, reg_or_imm) => write!(f, "\tCMN\t\t{}, {}", reg, reg_or_imm),

            Instruction::B(label) => write!(f, "\tB\t\t{}", label),
            Instruction::BCC(label) => write!(f, "\tBCC\t\t{}", label),
            Instruction::BCS(label) => write!(f, "\tBCS\t\t{}", label),
            Instruction::BEQ(label) => write!(f, "\tBEQ\t\t{}", label),
            Instruction::BGE(label) => write!(f, "\tBGE\t\t{}", label),
            Instruction::BGT(label) => write!(f, "\tBGT\t\t{}", label),
            Instruction::BHI(label) => write!(f, "\tBHI\t\t{}", label),
            Instruction::BHS(label) => write!(f, "\tBHS\t\t{}", label),
            Instruction::BL(label) => write!(f, "\tBL\t\t{}", label),
            Instruction::BLE(label) => write!(f, "\tBLE\t\t{}", label),
            Instruction::BLO(label) => write!(f, "\tBLO\t\t{}", label),
            Instruction::BLS(label) => write!(f, "\tBLS\t\t{}", label),
            Instruction::BLT(label) => write!(f, "\tBLT\t\t{}", label),
            Instruction::BMI(label) => write!(f, "\tBMI\t\t{}", label),
            Instruction::BNE(label) => write!(f, "\tBNE\t\t{}", label),
            Instruction::BPL(label) => write!(f, "\tBPL\t\t{}", label),
            Instruction::BVC(label) => write!(f, "\tBVC\t\t{}", label),
            Instruction::BVS(label) => write!(f, "\tBVS\t\t{}", label),
            Instruction::BLX(reg) => write!(f, "\tBLX\t\t{}", reg),
            Instruction::BX(reg) => write!(f, "\tBX\t\t{}", reg),

            Instruction::MOV(reg, reg_or_imm) => write!(f, "\tMOV\t\t{}, {}", reg, reg_or_imm),
            Instruction::MOVS(reg, reg_or_imm) => write!(f, "\tMOVS\t\t{}, {}", reg, reg_or_imm),
            Instruction::MVNS(reg, reg1) => write!(f, "\tMVNS\t\t{}, {}", reg, reg1),
            Instruction::MRS(reg, spec_reg) => write!(f, "\tMRS\t\t{}, {}", reg, spec_reg),
            Instruction::MSR(spec_reg, reg) => write!(f, "\tMSR\t\t{}, {}", spec_reg, reg),

            Instruction::REV(reg, reg1) => write!(f, "\tREV\t\t{}, {}", reg, reg1),
            Instruction::REV16(reg, reg1) => write!(f, "\tREV16\t\t{}, {}", reg, reg1),
            Instruction::REVSH(reg, reg1) => write!(f, "\tREVSH\t\t{}, {}", reg, reg1),

            Instruction::SXTB(reg, reg1) => write!(f, "\tSXTB\t\t{}, {}", reg, reg1),
            Instruction::SXTH(reg, reg1) => write!(f, "\tSXTH\t\t{}, {}", reg, reg1),
            Instruction::UXTB(reg, reg1) => write!(f, "\tUXTB\t\t{}, {}", reg, reg1),
            Instruction::UXTH(reg, reg1) => write!(f, "\tUXTH\t\t{}, {}", reg, reg1),

            Instruction::NOP => write!(f, "\tNOP"),
            _ => todo!(),
        }
    }
}
