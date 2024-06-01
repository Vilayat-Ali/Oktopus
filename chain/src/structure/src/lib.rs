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
    }

    pub fn substract(&mut self, big_num: &BigNum) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn big_number_addition() {
        assert_eq!(2, 1+1);
    }

    #[test]
    fn big_number_substraction() {
        assert_eq!(0, 1-1);
    }
}