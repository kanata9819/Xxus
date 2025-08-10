use dioxus::prelude::*;
use crate::enums::enum_global::FlowType;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CashFlows {
    pub flows: Vec<CashFlow>,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CashFlow {
    pub id: i32,
    pub amount: i32,
    pub name: String,
    pub flow: FlowType,
}

