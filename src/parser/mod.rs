use roxmltree as xml;

mod immediate;
mod instruction;
mod operand;
mod program;
mod register;

pub use immediate::Immediate;
pub use instruction::{Instruction, Label};
pub use program::Function;
pub use program::Program;
pub use register::{Register, RegisterList, SpecialRegister};

pub fn gen_ast(root: xml::Node) -> Function {
    let main = match root.children().find(|func| func.has_tag_name("main")) {
        Some(i) => i,
        None => todo!("missing main function"),
    };

    // Program {
    //     functions: root.children().map(block_ast).collect(),
    // }

    block_ast(main)
}

pub fn block_ast(parent: xml::Node) -> Function {
    let mut instructions = Vec::new();
    let mut labels = Vec::new();

    for (idx, instruction) in parent.children().filter(xml::Node::is_element).enumerate() {
        match instruction.tag_name().name() {
            "adcs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ADCS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ADCS(rd, rd, rn));
                }
            }

            "add" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ADD(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::ADD(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ADD(rd, rd, rn.into()));
                }
            }

            "adds" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ADDS(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::ADDS(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ADDS(rd, rd, rn.into()));
                }
            }

            "muls" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                instructions.push(Instruction::MULS(rd, rn));
            }

            "sub" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::SUB(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::SUB(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::SUB(rd, rd, rn.into()));
                }
            }

            "subs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::SUBS(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::SUBS(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::SUBS(rd, rd, rn.into()));
                }
            }

            "rsbs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::RSBS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::RSBS(rd, rd, rn.into()));
                }
            }

            "sbcs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = match instruction.attribute("rn") {
                    Some(i) => i.try_into().unwrap(),
                    None => rd,
                };

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::SBCS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::SBCS(rd, rd, rn));
                }
            }

            "ands" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ANDS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ANDS(rd, rd, rn));
                }
            }

            "eors" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::EORS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::EORS(rd, rd, rn));
                }
            }

            "bics" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::BICS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::BICS(rd, rd, rn));
                }
            }

            "orrs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ORRS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ORRS(rd, rd, rn));
                }
            }

            "tst" => {
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::TST(rn, rm));
            }

            "asrs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::ASRS(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::ASRS(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::ASRS(rd, rd, rn.into()));
                }
            }

            "lsls" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::LSLS(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::LSLS(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::LSLS(rd, rd, rn.into()));
                }
            }

            "lsrs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::LSRS(rd, rn, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::LSRS(rd, rn, imm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::LSRS(rd, rd, rn.into()));
                }
            }

            "rors" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::RORS(rd, rn, rm.try_into().unwrap()));
                } else {
                    instructions.push(Instruction::RORS(rd, rd, rn));
                }
            }

            "adr" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let label = instruction.attribute("label").unwrap().to_string();

                instructions.push(Instruction::ADR(rd, label));
            }

            "ldmia" => {
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                let reglist = instruction
                    .attribute("reglist")
                    .unwrap()
                    .try_into()
                    .unwrap();
                let write_back = instruction.attribute("writeback").is_some();

                instructions.push(Instruction::LDMIA(rn, reglist, write_back))
            }

            "ldr" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::LDR(rt, addr))
            }

            "ldrb" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::LDRB(rt, addr))
            }

            "ldrh" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::LDRH(rt, addr))
            }

            "ldrsb" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::LDRSB(rt, addr))
            }

            "ldrsh" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::LDRSH(rt, addr))
            }

            "stmia" => {
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                let reglist = instruction
                    .attribute("reglist")
                    .unwrap()
                    .try_into()
                    .unwrap();
                let write_back = instruction.attribute("writeback").is_some();

                instructions.push(Instruction::STMIA(rn, reglist, write_back))
            }

            "str" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::STR(rt, addr))
            }

            "strb" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::STRB(rt, addr))
            }

            "strh" => {
                let rt = instruction.attribute("rt").unwrap().try_into().unwrap();
                let addr = instruction.attribute("addr").unwrap().try_into().unwrap();

                instructions.push(Instruction::STRH(rt, addr))
            }

            "pop" => instructions.push(Instruction::POP(
                instruction
                    .attribute("reglist")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )),

            "push" => instructions.push(Instruction::PUSH(
                instruction
                    .attribute("reglist")
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )),

            "cmp" => {
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();

                if let Some(i) = instruction.attribute("rm") {
                    instructions.push(Instruction::CMP(rn, i.try_into().unwrap()));
                } else if let Some(i) = instruction.attribute("imm") {
                    instructions.push(Instruction::CMP(rn, i.try_into().unwrap()));
                }

                todo!()
            }

            "cmn" => {
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::CMN(rn, rm));
            }

            "b" => instructions.push(Instruction::B(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bcc" => instructions.push(Instruction::BCC(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bcs" => instructions.push(Instruction::BCS(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "beq" => instructions.push(Instruction::BEQ(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bge" => instructions.push(Instruction::BGE(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bgt" => instructions.push(Instruction::BGT(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bhi" => instructions.push(Instruction::BHI(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bhs" => instructions.push(Instruction::BHS(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bl" => instructions.push(Instruction::BL(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "ble" => instructions.push(Instruction::BLE(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "blo" => instructions.push(Instruction::BLO(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bls" => instructions.push(Instruction::BLS(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "blt" => instructions.push(Instruction::BLT(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bmi" => instructions.push(Instruction::BMI(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bne" => instructions.push(Instruction::BNE(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bpl" => instructions.push(Instruction::BPL(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bvc" => instructions.push(Instruction::BVC(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "bvs" => instructions.push(Instruction::BVS(
                instruction.attribute("label").unwrap().to_string(),
            )),
            "blx" => instructions.push(Instruction::BLX(
                instruction.attribute("rm").unwrap().try_into().unwrap(),
            )),
            "bx" => instructions.push(Instruction::BX(
                instruction.attribute("rm").unwrap().try_into().unwrap(),
            )),

            "mov" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::MOV(rd, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::MOV(rd, imm.try_into().unwrap()));
                } else {
                    todo!()
                }
            }

            "movs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();

                if let Some(rm) = instruction.attribute("rm") {
                    instructions.push(Instruction::MOVS(rd, rm.try_into().unwrap()));
                } else if let Some(imm) = instruction.attribute("imm") {
                    instructions.push(Instruction::MOVS(rd, imm.try_into().unwrap()));
                } else {
                    todo!()
                }
            }

            "mvns" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();
                instructions.push(Instruction::MVNS(rd, rm))
            }

            "mrs" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let spec_reg = instruction
                    .attribute("spec_reg")
                    .unwrap()
                    .try_into()
                    .unwrap();
                instructions.push(Instruction::MRS(rd, spec_reg));
            }

            "msr" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let spec_reg = instruction
                    .attribute("spec_reg")
                    .unwrap()
                    .try_into()
                    .unwrap();
                instructions.push(Instruction::MSR(spec_reg, rd));
            }

            "rev" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                instructions.push(Instruction::REV(rd, rn));
            }

            "rev16" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                instructions.push(Instruction::REV16(rd, rn));
            }

            "revsh" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rn = instruction.attribute("rn").unwrap().try_into().unwrap();
                instructions.push(Instruction::REVSH(rd, rn));
            }

            "sxtb" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::SXTB(rd, rm));
            }

            "sxth" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::SXTH(rd, rm));
            }

            "uxtb" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::UXTB(rd, rm));
            }

            "uxth" => {
                let rd = instruction.attribute("rd").unwrap().try_into().unwrap();
                let rm = instruction.attribute("rm").unwrap().try_into().unwrap();

                instructions.push(Instruction::UXTH(rd, rm));
            }

            "svc" => instructions.push(Instruction::SVC(
                instruction.attribute("imm").unwrap().try_into().unwrap(),
            )),
            "bkpt" => instructions.push(Instruction::BKPT(
                instruction.attribute("imm").unwrap().try_into().unwrap(),
            )),

            "cpsid" => instructions.push(Instruction::CPSID),
            "cpsie" => instructions.push(Instruction::CPSIE),
            "dmb" => instructions.push(Instruction::DMB),
            "dsb" => instructions.push(Instruction::DSB),
            "isb" => instructions.push(Instruction::ISB),
            "nop" => instructions.push(Instruction::NOP),
            "sev" => instructions.push(Instruction::SEV),
            "wfe" => instructions.push(Instruction::WFE),
            "wfi" => instructions.push(Instruction::WFI),

            label => {
                labels.push((label.to_string(), idx));
                let f = block_ast(instruction);

                instructions.extend(f.instructions);
                labels.extend(f.labels.into_iter().map(|(s, i)| (s, i + idx + 1)));
            }
        }
    }

    Function {
        instructions,
        labels,
    }
}
