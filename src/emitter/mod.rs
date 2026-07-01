use std::io::Write;

use crate::parser::{Function, Instruction};

pub fn write_basic(main: Function) {
    let instructions_iter = main.instructions.into_iter();
    let mut labels_iter = main.labels.iter();
    let mut next_label = labels_iter.next();

    let mut f = std::fs::File::create("./main.s").unwrap();

    writeln!(
        f,
        r#"	AREA	Reset, DATA, READONLY
	EXPORT	__Vectors

__Vectors
	DCD	0x20001000
	DCD	Reset_Handler

	AREA	|.text|, CODE, READONLY
	THUMB
	EXPORT	Reset_Handler

Reset_Handler PROC"#
    )
    .unwrap();
    for (index, instruction) in instructions_iter.enumerate() {
        if next_label.is_some_and(|(_, idx)| *idx == index) {
            let (label_name, _) = next_label.unwrap();
            f.write(label_name.as_bytes()).unwrap();
            f.write("\n".as_bytes()).unwrap();

            next_label = labels_iter.next();
        }

        match instruction {
            Instruction::ADCS(register, register1, register2) => {
                writeln!(f, "\tADCS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::ADD(register, register1, register2) => {
                writeln!(f, "\tADD\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::ADDS(register, register1, register2) => {
                writeln!(f, "\tADDS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::MULS(register, register1) => {
                writeln!(f, "\tMULS\t\t{}, {}, {}", register, register, register1)
            }
            Instruction::SUB(register, register1, register2) => {
                writeln!(f, "\tSUB\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::SUBS(register, register1, register2) => {
                writeln!(f, "\tSUBS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::RSBS(register, register1, register2) => {
                writeln!(f, "\tRSBS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::SBCS(register, register1, register2) => {
                writeln!(f, "\tSBCS\t\t{}, {}, {}", register, register1, register2)
            }

            Instruction::ANDS(register, register1, register2) => {
                writeln!(f, "\tANDS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::EORS(register, register1, register2) => {
                writeln!(f, "\tEORS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::BICS(register, register1, register2) => {
                writeln!(f, "\tBICS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::ORRS(register, register1, register2) => {
                writeln!(f, "\tORRS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::TST(register, register1) => {
                writeln!(f, "\tTST\t\t{}, {}", register, register1)
            }
            Instruction::ASRS(register, register1, register2) => {
                writeln!(f, "\tASRS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::LSLS(register, register1, register2) => {
                writeln!(f, "\tLSLS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::LSRS(register, register1, register2) => {
                writeln!(f, "\tLSRS\t\t{}, {}, {}", register, register1, register2)
            }
            Instruction::RORS(register, register1, register2) => {
                writeln!(f, "\tRORS\t\t{}, {}, {}", register, register1, register2)
            }

            Instruction::ADR(register, label) => writeln!(f, "\tADR\t\t{}, {}", register, label),
            Instruction::LDMIA(register, reg_list, write_back) => writeln!(
                f,
                "\tLDMIA\t\t{}{}, {}",
                register,
                if write_back { '!' } else { ' ' },
                reg_list
            ),
            Instruction::LDR(reg, addr) => writeln!(f, "\tLDR\t\t{}, {}", reg, addr),
            Instruction::LDRB(reg, addr) => writeln!(f, "\tLDRB\t\t{}, {}", reg, addr),
            Instruction::LDRH(reg, addr) => writeln!(f, "\tLDRH\t\t{}, {}", reg, addr),
            Instruction::LDRSB(reg, addr) => writeln!(f, "\tLDRSB\t\t{}, {}", reg, addr),
            Instruction::LDRSH(reg, addr) => writeln!(f, "\tLDRSH\t\t{}, {}", reg, addr),
            Instruction::STMIA(register, reg_list, write_back) => writeln!(
                f,
                "\tSTMIA\t\t{}{}, {}",
                register,
                if write_back { '!' } else { ' ' },
                reg_list
            ),
            Instruction::STR(reg, addr) => writeln!(f, "\tSTR\t\t{}, {}", reg, addr),
            Instruction::STRB(reg, addr) => writeln!(f, "\tSTRB\t\t{}, {}", reg, addr),
            Instruction::STRH(reg, addr) => writeln!(f, "\tSTRH\t\t{}, {}", reg, addr),

            Instruction::POP(reg_list) => writeln!(f, "\tPOP\t\t{}", reg_list),
            Instruction::PUSH(reg_list) => writeln!(f, "\tPUSH\t\t{}", reg_list),

            Instruction::CMP(reg, reg_or_imm) => writeln!(f, "\tCMP\t\t{}, {}", reg, reg_or_imm),
            Instruction::CMN(reg, reg_or_imm) => writeln!(f, "\tCMN\t\t{}, {}", reg, reg_or_imm),

            Instruction::B(label) => writeln!(f, "\tB\t\t{}", label),
            Instruction::BCC(label) => writeln!(f, "\tBCC\t\t{}", label),
            Instruction::BCS(label) => writeln!(f, "\tBCS\t\t{}", label),
            Instruction::BEQ(label) => writeln!(f, "\tBEQ\t\t{}", label),
            Instruction::BGE(label) => writeln!(f, "\tBGE\t\t{}", label),
            Instruction::BGT(label) => writeln!(f, "\tBGT\t\t{}", label),
            Instruction::BHI(label) => writeln!(f, "\tBHI\t\t{}", label),
            Instruction::BHS(label) => writeln!(f, "\tBHS\t\t{}", label),
            Instruction::BL(label) => writeln!(f, "\tBL\t\t{}", label),
            Instruction::BLE(label) => writeln!(f, "\tBLE\t\t{}", label),
            Instruction::BLO(label) => writeln!(f, "\tBLO\t\t{}", label),
            Instruction::BLS(label) => writeln!(f, "\tBLS\t\t{}", label),
            Instruction::BLT(label) => writeln!(f, "\tBLT\t\t{}", label),
            Instruction::BMI(label) => writeln!(f, "\tBMI\t\t{}", label),
            Instruction::BNE(label) => writeln!(f, "\tBNE\t\t{}", label),
            Instruction::BPL(label) => writeln!(f, "\tBPL\t\t{}", label),
            Instruction::BVC(label) => writeln!(f, "\tBVC\t\t{}", label),
            Instruction::BVS(label) => writeln!(f, "\tBVS\t\t{}", label),
            Instruction::BLX(reg) => writeln!(f, "\tBLX\t\t{}", reg),
            Instruction::BX(reg) => writeln!(f, "\tBX\t\t{}", reg),

            Instruction::MOV(reg, reg_or_imm) => writeln!(f, "\tMOV\t\t{}, {}", reg, reg_or_imm),
            Instruction::MOVS(reg, reg_or_imm) => writeln!(f, "\tMOVS\t\t{}, {}", reg, reg_or_imm),
            Instruction::MVNS(reg, reg1) => writeln!(f, "\tMVNS\t\t{}, {}", reg, reg1),
            Instruction::MRS(reg, spec_reg) => writeln!(f, "\tMRS\t\t{}, {}", reg, spec_reg),
            Instruction::MSR(spec_reg, reg) => writeln!(f, "\tMSR\t\t{}, {}", spec_reg, reg),

            Instruction::REV(reg, reg1) => writeln!(f, "\tREV\t\t{}, {}", reg, reg1),
            Instruction::REV16(reg, reg1) => writeln!(f, "\tREV16\t\t{}, {}", reg, reg1),
            Instruction::REVSH(reg, reg1) => writeln!(f, "\tREVSH\t\t{}, {}", reg, reg1),

            Instruction::SXTB(reg, reg1) => writeln!(f, "\tSXTB\t\t{}, {}", reg, reg1),
            Instruction::SXTH(reg, reg1) => writeln!(f, "\tSXTH\t\t{}, {}", reg, reg1),
            Instruction::UXTB(reg, reg1) => writeln!(f, "\tUXTB\t\t{}, {}", reg, reg1),
            Instruction::UXTH(reg, reg1) => writeln!(f, "\tUXTH\t\t{}, {}", reg, reg1),

            Instruction::NOP => writeln!(f, "\tNOP"),
            _ => todo!(),
        }
        .unwrap();
    }

    writeln!(f, "End_Loop\n\tB\t\tEnd_Loop\n\tENDP\n\n\tALIGN\n\tEND").unwrap();
}
