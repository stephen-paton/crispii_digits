/// An enum representing all of the possible permutations of the octal (base 8) digit 
#[derive(Clone, Copy, Debug)]
pub enum Oct {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<Oct> for u8 {
    fn from(value: Oct) -> Self {
        match value {
            Oct::Zero => 0,
            Oct::One => 1,
            Oct::Two => 2,
            Oct::Three => 3,
            Oct::Four => 4,
            Oct::Five => 5,
            Oct::Six => 6,
            Oct::Seven => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Oct::Zero);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Oct::Seven);
        assert_eq!(result, 7);
    }
}
