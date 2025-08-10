use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CashFlow {
    pub id: i32,
    pub amount: i32,
    pub name: String,
    pub flow: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  AddCashFlowProps {
    pub name: String,
    pub amount: i32,
    pub flow_type: String
}

/// 親に渡すデータ（必要なら serde を付けてバックエンドへ）
// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkRecord {
    pub date: String,       // "2025-08-08"
    pub start_time: String, // "09:00"
    pub end_time: String,   // "18:00"
    pub rest_time: String,  // "00:00"
    pub hourly_wage: i32,   // 円
    pub minutes: i32,       // 勤務時間(分)
    pub amount: i32,        // 概算支給額(円) = 時給 * 分 / 60
    pub note: String,
}
