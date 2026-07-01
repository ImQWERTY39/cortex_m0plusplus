#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
}

impl Register {
    pub fn is_low_register(&self) -> bool {
        *self < Self::R8
    }

    pub fn is_high_register(&self) -> bool {
        *self > Self::R7
    }

    pub fn as_num(&self) -> u8 {
        match self {
            Self::R0 => 0,
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 8,
            Self::R8 => 8,
            Self::R9 => 9,
            Self::R10 => 10,
            Self::R11 => 11,
            Self::R12 => 12,
            Self::SP => 13,
            Self::LR => 14,
            Self::PC => 15,
        }
    }

    pub fn from_num(num: u8) -> Self {
        match num % 16 {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            8 => Self::R8,
            9 => Self::R9,
            10 => Self::R10,
            11 => Self::R11,
            12 => Self::R12,
            13 => Self::SP,
            14 => Self::LR,
            15 => Self::PC,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "R0" => Ok(Self::R0),
            "R1" => Ok(Self::R1),
            "R2" => Ok(Self::R2),
            "R3" => Ok(Self::R3),
            "R4" => Ok(Self::R4),
            "R5" => Ok(Self::R5),
            "R6" => Ok(Self::R6),
            "R7" => Ok(Self::R7),
            "R8" => Ok(Self::R8),
            "R9" => Ok(Self::R9),
            "R10" => Ok(Self::R10),
            "R11" => Ok(Self::R11),
            "R12" => Ok(Self::R12),
            "SP" => Ok(Self::SP),
            "LR" => Ok(Self::LR),
            "PC" => Ok(Self::PC),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::R0 => "R0",
                Self::R1 => "R1",
                Self::R2 => "R2",
                Self::R3 => "R3",
                Self::R4 => "R4",
                Self::R5 => "R5",
                Self::R6 => "R6",
                Self::R7 => "R7",
                Self::R8 => "R8",
                Self::R9 => "R9",
                Self::R10 => "R10",
                Self::R11 => "R11",
                Self::R12 => "R12",
                Self::SP => "SP",
                Self::LR => "LR",
                Self::PC => "PC",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpecialRegister {
    MSP,
    PSP,
    PRIMASK,
    CONTROL,
    XPSR,
}

impl From<&str> for SpecialRegister {
    fn from(value: &str) -> Self {
        match value {
            "MSP" => Self::MSP,
            "PSP" => Self::PSP,
            "PRIMASK" => Self::PRIMASK,
            "CONTROL" => Self::CONTROL,
            "xPSR" => Self::XPSR,
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for SpecialRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MSP => "MSP",
                Self::PSP => "PSP",
                Self::PRIMASK => "PRIMASK",
                Self::CONTROL => "CONTROL",
                Self::XPSR => "xPSR",
            }
        )
    }
}

#[derive(Debug)]
pub struct RegisterList(u16);

impl RegisterList {
    const BANNED_PUSH_BITS: u16 = 0xBF00;
    const BANNED_POP_BITS: u16 = 0x7F00;

    pub fn is_only_low_regs(&self) -> bool {
        (self.0 & !0x00FF) == 0
    }

    pub fn is_valid_push(&self) -> bool {
        (self.0 & Self::BANNED_PUSH_BITS) == 0
    }

    pub fn is_valid_pop(&self) -> bool {
        (self.0 & Self::BANNED_POP_BITS) == 0
    }
}

impl TryFrom<&str> for RegisterList {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut regs = 0;
        for register in value.split(",").map(|string| string.trim()) {
            if let Ok(i) = Register::try_from(register) {
                regs |= 1 << i.as_num();
                continue;
            }

            if let Some((start, end)) = register.split_once("-") {
                if let (Ok(s), Ok(e)) = (
                    Register::try_from(start.trim()),
                    Register::try_from(end.trim()),
                ) {
                    let start_num = s.as_num();
                    let end_num = e.as_num();

                    if start > end {
                        return Err(());
                    }

                    for i in start_num..=end_num {
                        regs |= 1 << i;
                    }
                    continue;
                }
            }

            return Err(());
        }

        Ok(Self(regs))
    }
}

impl std::fmt::Display for RegisterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for i in 0..16 {
            if (self.0 & (1 << i)) != 0 {
                write!(f, "{}", Register::from_num(i))?;
            }

            if i != 15 {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}")
    }
}
