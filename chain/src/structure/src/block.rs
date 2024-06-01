use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, Result};

use crate::txn::Txn;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    txn: HashMap<String, Txn>,
    pub mined_by: String,
    pub minted_at: String,
}

impl Block {
    pub fn get_txns(&self) -> &HashMap<String, Txn> {
        &self.txn
    }

    pub fn to_json(&self) -> Result<String> {
        to_string_pretty(&self)
    }
}