/// An enum representing all of the possible permutations of the hexadecimal (base 16) digit 
#[derive(Clone, Copy, Debug)]
pub enum Hex {
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
    A,
    B,
    C,
    D,
    E,
    F
}

impl From<Hex> for u8 {
    fn from(value: Hex) -> Self {
        match value {
            Hex::Zero => 0,
            Hex::One => 1,
            Hex::Two => 2,
            Hex::Three => 3,
            Hex::Four => 4,
            Hex::Five => 5,
            Hex::Six => 6,
            Hex::Seven => 7,
            Hex::Eight => 8,
            Hex::Nine => 9,
            Hex::A => 10,
            Hex::B => 11,
            Hex::C => 12,
            Hex::D => 13,
            Hex::E => 14,
            Hex::F => 15,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Hex::Zero);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Hex::F);
        assert_eq!(result, 15);
    }
}
