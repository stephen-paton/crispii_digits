/// An enum representing all of the possible permutations of the binary (base 2) digit 
#[derive(Clone, Copy, Debug)]
pub enum Bin {
    Zero,
    One,
}

impl From<Bin> for u8 {
    fn from(value: Bin) -> Self {
        match value {
            Bin::Zero => 0,
            Bin::One => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(Bin::Zero);
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(Bin::One);
        assert_eq!(result, 1);
    }
}
