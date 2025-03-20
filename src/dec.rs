use std::fmt::Display;

/// An enum representing all of the possible permutations of the decimal (base 10) digit 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Dec {
    #[default]
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

impl Display for Dec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dec::D0 => write!(f, "Dec:0"),
            Dec::D1 => write!(f, "Dec:1"),
            Dec::D2 => write!(f, "Dec:2"),
            Dec::D3 => write!(f, "Dec:3"),
            Dec::D4 => write!(f, "Dec:4"),
            Dec::D5 => write!(f, "Dec:5"),
            Dec::D6 => write!(f, "Dec:6"),
            Dec::D7 => write!(f, "Dec:7"),
            Dec::D8 => write!(f, "Dec:8"),
            Dec::D9 => write!(f, "Dec:9"),
        }
    }
}

impl From<Dec> for u8 {
    fn from(value: Dec) -> Self {
        match value {
            Dec::D0 => 0,
            Dec::D1 => 1,
            Dec::D2 => 2,
            Dec::D3 => 3,
            Dec::D4 => 4,
            Dec::D5 => 5,
            Dec::D6 => 6,
            Dec::D7 => 7,
            Dec::D8 => 8,
            Dec::D9 => 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Dec::D0);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Dec::D9);
        assert_eq!(result, 9);
    }
}
