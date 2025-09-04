use serde::Deserialize;

#[derive(Deserialize)]
pub struct PayPayCsvRow {
    #[serde(rename = "取引日")]
    date: String,
    #[serde(rename = "出金金額（円）", deserialize_with = "de_amount_opt")]
    debit_jpy: Option<i64>,
    #[serde(rename = "入金金額（円）", deserialize_with = "de_amount_opt")]
    credit_jpy: Option<i64>,
    #[serde(rename = "海外出金金額", deserialize_with = "de_amount_opt")]
    debit_fx: Option<i64>,
    #[serde(rename = "通貨")]
    currency: Option<String>,
    #[serde(
        rename = "変換レート（円)",
        alias = "変換レート（円）",
        deserialize_with = "de_rate_opt"
    )]
    fx_rate_jpy: Option<f64>,
    #[serde(rename = "利用国")]
    country: Option<String>,
    #[serde(rename = "取引内容")]
    description: Option<String>,
    #[serde(rename = "取引先")]
    counterparty: Option<String>,
    #[serde(rename = "取引方法")]
    method: Option<String>,
    #[serde(rename = "支払い区分")]
    category: Option<String>,
    #[serde(rename = "利用者")]
    user: Option<String>,
    #[serde(rename = "取引番号")]
    txn_id: Option<String>,
}

fn de_amount_opt<'de, D>(d: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(d)?;
    Ok(s.and_then(|x| {
        let cleaned = x.replace(',', "").replace('円', "").trim().to_string();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    })
    .and_then(|clean| clean.parse::<i64>().ok()))
}

fn de_rate_opt<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(d)?;
    Ok(s.and_then(|x| {
        let cleaned = x.replace(',', "").replace('円', "").trim().to_string();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    })
    .and_then(|clean| clean.parse::<f64>().ok()))
}
