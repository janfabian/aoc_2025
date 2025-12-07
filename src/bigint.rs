use num_bigint::{BigInt, Sign};

pub trait BigIntExt {
    fn num_of_digits(&self) -> usize;
    fn to_vec_of_digits(&self) -> Vec<u32>;
    fn get_digit_from_right(&self, position: usize) -> Result<u32, String> {
        let digits = self.to_vec_of_digits();
        if position >= digits.len() {
            Err("Position out of bounds".into())
        } else {
            Ok(digits[digits.len() - 1 - position])
        }
    }
    fn from_vec_of_digits(digits: &[u32]) -> Self;
}

impl BigIntExt for BigInt {
    fn num_of_digits(&self) -> usize {
        self.to_string().len()
    }

    fn to_vec_of_digits(&self) -> Vec<u32> {
        self.to_u32_digits().1
    }

    fn from_vec_of_digits(digits: &[u32]) -> Self {
        String::from_iter(
            digits
                .iter()
                .map(|d| std::char::from_digit(*d, 10).unwrap()),
        )
        .parse::<BigInt>()
        .unwrap()
    }
}
