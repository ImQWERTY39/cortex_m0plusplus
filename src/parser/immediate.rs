#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Immediate(u32);

impl TryFrom<&str> for Immediate {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<u32>() {
            Ok(Self(i))
        } else if let Ok(i) = value.parse::<i32>() {
            Ok(Self(i.cast_unsigned()))
        } else {
            Err(())
        }
    }
}

impl From<u32> for Immediate {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<i32> for Immediate {
    fn from(value: i32) -> Self {
        Self(value.cast_unsigned())
    }
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
