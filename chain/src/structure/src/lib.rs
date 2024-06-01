pub mod txn;
pub mod block;
pub mod chain;
pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BigNum {
    pub int_val: Vec<u8>,
    pub frac_val: [u8; 4]
}

impl std::fmt::Debug for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        writeln!(f, "{}.{}", 
        self.int_val.iter().map(|x| x.to_string()).collect::<String>(), 
        self.frac_val.iter().map(|x| x.to_string()).collect::<String>())
    }
}

impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}.{}", 
        self.int_val.iter().map(|x| x.to_string()).collect::<String>(), 
        self.frac_val.iter().map(|x| x.to_string()).collect::<String>())
    }
}

impl Into<String> for BigNum {
    fn into(self) -> String {
        format!("{}.{}",
        self.int_val.iter().map(|x| x.to_string()).collect::<String>(), 
        self.frac_val.iter().map(|x| x.to_string()).collect::<String>())
    }
}

impl From<String> for BigNum {
    fn from(value: String) -> Self {
        let parts = value.split(".").map(|part| part.to_string()).collect::<Vec<String>>();

        if parts.len() != 2 {
            panic!("Fatal: Cannot convert the string `{}` into big number format.", value);
        }

        let int_val = parts[0].chars().map(|digit| digit.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    
        let frac_val = {
            let mut val_vec: [u8; 4] = [0; 4];
            parts[1].chars().enumerate().for_each(|(index, digit)| val_vec[index] = digit.to_digit(10).unwrap() as u8);
            val_vec
        };

        BigNum {
            int_val,
            frac_val
        }
    }
}

impl From<&str> for BigNum {
    fn from(value: &str) -> Self {
        let parts = value.split(".").map(|part| part.to_string()).collect::<Vec<String>>();

        if parts.len() != 2 {
            panic!("Fatal: Cannot convert the string `{}` into big number format.", value);
        }

        let int_val = parts[0].chars().map(|digit| digit.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    
        let frac_val = {
            let mut val_vec: [u8; 4] = [0; 4];
            parts[1].chars().enumerate().for_each(|(index, digit)| val_vec[index] = digit.to_digit(10).unwrap() as u8);
            val_vec
        };

        BigNum {
            int_val,
            frac_val
        }
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        if self.int_val.len() != other.int_val.len() {
            return false;
        }

        for idx in 0..4 {
            if self.frac_val[idx] != other.frac_val[idx]  {
                return false;
            }
        }

        for idx in 0..self.int_val.len() {
           if self.int_val[idx] != other.int_val[idx] {
                return false;
           } 
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl BigNum {
    pub fn new(int_val: Vec<u8>, frac_val: [u8; 4]) -> Self {
        Self {
            int_val,
            frac_val
        }
    }

    pub fn add(&mut self, big_num: &BigNum) {
        let mut is_carry = false;

        for idx in (0..4).rev() {
            let mut digit = self.frac_val[idx] + big_num.frac_val[idx] + (is_carry as u8);
            is_carry = false;

            if digit > 9 {
                digit = digit % 10;
                is_carry = true;
            }

            self.frac_val[idx] = digit;
        }

        let size = self.int_val.len().max(big_num.int_val.len());
        let self_int_vec_size = self.int_val.len();

        for idx in 1..=size {
            let digit_1 = match self_int_vec_size >= idx {
                true => self.int_val[self_int_vec_size - idx],
                false => 0
            };
            let digit_2 = match big_num.int_val.len() >= idx {
                true => big_num.int_val[big_num.int_val.len() - idx],
                false => 0
            };

            let mut digit_sum = digit_1 + digit_2 + (is_carry as u8);
            is_carry = false;

            if digit_sum > 9 {
                digit_sum = digit_sum % 10;
                is_carry = true;
            }
            
            match self_int_vec_size >= idx {
                true => self.int_val[self_int_vec_size - idx] = digit_sum,
                false => self.int_val.insert(0, digit_sum)
            }
        }
    }

    pub fn substract(&mut self, big_num: &BigNum) {
        
    }
}

#[cfg(test)]
mod tests {
    use crate::BigNum;

    #[test]
    fn big_number_addition() {
        let mut big_num_1 = BigNum::from("12345.6789");
        let big_num_2 = BigNum::from("0.6789");
        big_num_1.add(&big_num_2);

        assert_eq!(big_num_1.to_string().as_str(), "12346.3578");
    }

    #[test]
    fn big_number_substraction() {
        assert_eq!(0, 1-1);
    }
}