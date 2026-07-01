use super::Immediate;
use super::Label;
use super::Register;

#[derive(Debug)]
pub enum RegOrImm {
    Register(Register),
    Immediate(Immediate),
}

impl TryFrom<&str> for RegOrImm {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = Immediate::try_from(value) {
            Ok(Self::Immediate(i))
        } else if let Ok(i) = Register::try_from(value) {
            Ok(Self::Register(i))
        } else {
            Err(())
        }
    }
}

impl std::fmt::Display for RegOrImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrImm::Register(register) => write!(f, "{}", register),
            RegOrImm::Immediate(immediate) => write!(f, "#{}", immediate),
        }
    }
}

impl From<Register> for RegOrImm {
    fn from(value: Register) -> Self {
        Self::Register(value)
    }
}

impl From<Immediate> for RegOrImm {
    fn from(value: Immediate) -> Self {
        Self::Immediate(value)
    }
}

#[derive(Debug)]
pub enum Address {
    Immediate(Immediate),
    Label(Label),
    RegisterRelative(Register, RegOrImm),
}

impl TryFrom<&str> for Address {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = Immediate::try_from(value) {
            Ok(Self::Immediate(i))
        } else if let Some((r, imm)) = value.split_once(",") {
            if let (Ok(reg), Ok(i)) = (r.try_into(), imm.try_into()) {
                Ok(Self::RegisterRelative(reg, i))
            } else {
                Err(())
            }
        } else {
            Ok(Self::Label(value.to_string()))
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Immediate(immediate) => write!(f, "={immediate}"),
            Address::Label(label) => write!(f, "={label}"),
            Address::RegisterRelative(register, reg_or_imm) => {
                write!(f, "[{register}, {reg_or_imm}]")
            }
        }
    }
}
