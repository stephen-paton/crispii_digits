use std::fmt::Display;

/// An enum representing all of the possible permutations of the octal (base 8) digit 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Oct {
    #[default]
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
}

impl Display for Oct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Oct::O0 => write!(f, "Oct:0"),
            Oct::O1 => write!(f, "Oct:1"),
            Oct::O2 => write!(f, "Oct:2"),
            Oct::O3 => write!(f, "Oct:3"),
            Oct::O4 => write!(f, "Oct:4"),
            Oct::O5 => write!(f, "Oct:5"),
            Oct::O6 => write!(f, "Oct:6"),
            Oct::O7 => write!(f, "Oct:7"),
        }
    }
}

impl From<Oct> for u8 {
    fn from(value: Oct) -> Self {
        match value {
            Oct::O0 => 0,
            Oct::O1 => 1,
            Oct::O2 => 2,
            Oct::O3 => 3,
            Oct::O4 => 4,
            Oct::O5 => 5,
            Oct::O6 => 6,
            Oct::O7 => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Oct::O0);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Oct::O7);
        assert_eq!(result, 7);
    }
}
