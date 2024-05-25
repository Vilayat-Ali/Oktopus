use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str, Result};

use crate::BigNum;

#[derive(Serialize, Deserialize, Debug)]
pub struct Txn {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub funds: BigNum,
    pub minted_at: String
}

impl From<String> for Txn {
    fn from(value: String) -> Self {
        from_str(&value).expect("Txn Error: Invalid Txn JSON parsed")
    }
}

impl Txn {
    pub fn to_json(&self) -> Result<String> {
        to_string_pretty(&self)
    }
}

pub struct TxnBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub funds: Option<BigNum>,
}

struct TxnPlaceholder {
    from: String,
    to: String,
    funds: BigNum,
    minted_at: String
}

impl TxnBuilder {
    pub fn new<S>(from: S, to: S) -> Self 
        where S: Into<String> {
        Self {
            from: Some(from.into()),
            to: Some(to.into()),
            funds: Some(BigNum::default()),
        }
    }

    pub fn set_funds<F>(&mut self, funds: F) -> &mut Self 
        where F: Into<BigNum> {
        self.funds = Some(funds.into());
        self
    }

    pub fn build(&mut self) -> Txn {
        let txn_placeholder = TxnPlaceholder {
            from: self.from.clone().expect("TxnBuilder Error: `from` field is empty but expected a value"),
            to: self.to.clone().expect("TxnBuilder Error: `to` field is empty but expected a value"),
            funds: self.funds.clone().expect("TxnBuilder Error: `funds` field is empty but expected a value"),
            minted_at: Utc::now().to_string()
        };

        let txn_hash = String::new();

        Txn {
            hash: txn_hash,
            from: txn_placeholder.from.clone(),
            to: txn_placeholder.to,
            funds: txn_placeholder.funds,
            minted_at: txn_placeholder.minted_at
        }
    }
}