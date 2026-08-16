use crate::parser::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionFlag {
    CarryClear,
    CarrySet,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    GreaterThan,
    Higher,
    HigherOrSame,
    LessThanOrEqual,
    LessThan,
    Lower,
    LowerOrSame,
    Minus,
    Plus,
    OverflowClear,
    OverflowSet,
}

impl ConditionFlag {
    pub fn invert(self) -> Self {
        match self {
            ConditionFlag::CarryClear => ConditionFlag::CarrySet,
            ConditionFlag::CarrySet => ConditionFlag::CarryClear,
            ConditionFlag::Equal => ConditionFlag::NotEqual,
            ConditionFlag::NotEqual => ConditionFlag::Equal,
            ConditionFlag::GreaterThanOrEqual => ConditionFlag::LessThan,
            ConditionFlag::GreaterThan => ConditionFlag::LessThanOrEqual,
            ConditionFlag::Higher => ConditionFlag::LowerOrSame,
            ConditionFlag::HigherOrSame => ConditionFlag::Lower,
            ConditionFlag::LessThanOrEqual => ConditionFlag::GreaterThan,
            ConditionFlag::LessThan => ConditionFlag::GreaterThanOrEqual,
            ConditionFlag::Lower => ConditionFlag::HigherOrSame,
            ConditionFlag::LowerOrSame => ConditionFlag::Higher,
            ConditionFlag::Minus => ConditionFlag::Plus,
            ConditionFlag::Plus => ConditionFlag::Minus,
            ConditionFlag::OverflowClear => ConditionFlag::OverflowSet,
            ConditionFlag::OverflowSet => ConditionFlag::OverflowClear,
        }
    }
}

impl TryFrom<&str> for ConditionFlag {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CC" => Ok(Self::CarryClear),
            "CS" => Ok(Self::CarrySet),
            "EQ" => Ok(Self::Equal),
            "NE" => Ok(Self::NotEqual),
            "GE" => Ok(Self::GreaterThanOrEqual),
            "GT" => Ok(Self::GreaterThan),
            "HI" => Ok(Self::Higher),
            "HS" => Ok(Self::HigherOrSame),
            "LE" => Ok(Self::LessThanOrEqual),
            "LO" => Ok(Self::Lower),
            "LS" => Ok(Self::LowerOrSame),
            "LT" => Ok(Self::LessThan),
            "MI" => Ok(Self::Minus),
            "PL" => Ok(Self::Plus),
            "VC" => Ok(Self::OverflowClear),
            "VS" => Ok(Self::OverflowSet),
            _ => Err(()),
        }
    }
}
