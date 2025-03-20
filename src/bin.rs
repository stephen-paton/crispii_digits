use std::fmt::Display;

/// An enum representing all of the possible permutations of the binary (base 2) digit 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Bin {
    #[default]
    B0,
    B1,
}

impl Display for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bin::B0 => write!(f, "Bin:0"),
            Bin::B1 => write!(f, "Bin:1"),
        }
    }
}

impl From<Bin> for u8 {
    fn from(value: Bin) -> Self {
        match value {
            Bin::B0 => 0,
            Bin::B1 => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Bin::B0);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Bin::B1);
        assert_eq!(result, 1);
    }
}
