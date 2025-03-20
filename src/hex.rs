use std::fmt::Display;

/// An enum representing all of the possible permutations of the hexadecimal (base 16) digit 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Hex {
    #[default]
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
    HA,
    HB,
    HC,
    HD,
    HE,
    HF,
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hex::H0 => write!(f, "Hex:0"),
            Hex::H1 => write!(f, "Hex:1"),
            Hex::H2 => write!(f, "Hex:2"),
            Hex::H3 => write!(f, "Hex:3"),
            Hex::H4 => write!(f, "Hex:4"),
            Hex::H5 => write!(f, "Hex:5"),
            Hex::H6 => write!(f, "Hex:6"),
            Hex::H7 => write!(f, "Hex:7"),
            Hex::H8 => write!(f, "Hex:8"),
            Hex::H9 => write!(f, "Hex:9"),
            Hex::HA => write!(f, "Hex:A"),
            Hex::HB => write!(f, "Hex:B"),
            Hex::HC => write!(f, "Hex:C"),
            Hex::HD => write!(f, "Hex:D"),
            Hex::HE => write!(f, "Hex:E"),
            Hex::HF => write!(f, "Hex:F"),
        }
    }
}

impl From<Hex> for u8 {
    fn from(value: Hex) -> Self {
        match value {
            Hex::H0 => 0,
            Hex::H1 => 1,
            Hex::H2 => 2,
            Hex::H3 => 3,
            Hex::H4 => 4,
            Hex::H5 => 5,
            Hex::H6 => 6,
            Hex::H7 => 7,
            Hex::H8 => 8,
            Hex::H9 => 9,
            Hex::HA => 10,
            Hex::HB => 11,
            Hex::HC => 12,
            Hex::HD => 13,
            Hex::HE => 14,
            Hex::HF => 15,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Hex::H0);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Hex::HF);
        assert_eq!(result, 15);
    }
}
