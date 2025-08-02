use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CashFlow {
    pub id: i32,
    pub amount: i32,
    pub name: String,
    pub flow: String,
    pub date: String,
}
