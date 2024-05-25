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

        let int_val = parts[0].chars().map(|digit| digit as u8).collect::<Vec<u8>>();
    
        let frac_val = {
            let mut val_vec: [u8; 4] = [0; 4];
            parts[1].chars().enumerate().for_each(|(index, digit)| val_vec[index] = digit as u8);
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

        let int_val = parts[0].chars().map(|digit| digit as u8).collect::<Vec<u8>>();
    
        let frac_val = {
            let mut val_vec: [u8; 4] = [0; 4];
            parts[1].chars().enumerate().for_each(|(index, digit)| val_vec[index] = digit as u8);
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
        self.int_val == other.int_val && self.frac_val == other.frac_val
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

        self.frac_val.iter_mut().enumerate().for_each(|(index, digit)| {
            let other_num_frac_val = big_num.frac_val[index];
            let sum_of_digits = other_num_frac_val + *digit;

            *digit = (sum_of_digits + (is_carry as u8)) % 10;

            match sum_of_digits > 9 {
                true => is_carry = true,
                false => is_carry = false
            }
        });

        self.int_val.iter_mut().enumerate().for_each(|(index, elem)| {

        });
    }

    pub fn substract(&mut self, big_num: &BigNum) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    fn generate_random_bignumber() -> BigNum {
        let int_val_vec: Vec<u8> = {
            let vec_len = thread_rng().gen_range(0..=10);
            let mut vec: Vec<u8> = Vec::with_capacity(vec_len);

            for idx in 0..=vec_len {
                vec[idx] = thread_rng().gen_range(0..10);
            }

            vec
        };

        let frac_val_arr: [u8; 4] = {
            let mut arr = [0; 4];

            arr.iter_mut().for_each(|x| *x = thread_rng().gen_range(0..10));

            arr
        };

        BigNum::new(int_val_vec, frac_val_arr)
    }
}