/// An enum representing all of the possible permutations of the decimal (base 10) digit 
#[derive(Clone, Copy, Debug)]
pub enum Dec {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<Dec> for u8 {
    fn from(value: Dec) -> Self {
        match value {
            Dec::Zero => 0,
            Dec::One => 1,
            Dec::Two => 2,
            Dec::Three => 3,
            Dec::Four => 4,
            Dec::Five => 5,
            Dec::Six => 6,
            Dec::Seven => 7,
            Dec::Eight => 8,
            Dec::Nine => 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Dec::Zero);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Dec::Nine);
        assert_eq!(result, 9);
    }
}
