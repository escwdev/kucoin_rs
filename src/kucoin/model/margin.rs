#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    pub granularity: i32,
    pub time_point: i64,
    pub value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginInfo {
    pub currency_list: Vec<String>,
    pub warning_debt_ratio: String,
    pub liq_debt_ratio: String,
    pub max_leverage: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccounts {
    pub accounts: Vec<MarginAccount>,
    pub debt_ratio: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccount {
    pub available_balance: String,
    pub currency: String,
    pub hold_balance: String,
    pub liability: String,
    pub max_borrow_size: String,
    pub total_balance: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowOrderId {
    pub order_id: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowOrder {
    pub currency: String,
    pub filled: String,
    pub match_list: Vec<MatchList>,
    pub order_id: String,
    pub size: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchList {
    pub currency: String,
    pub daily_int_rate: String,
    pub size: String,
    pub term: i32,
    pub timestamp: i64,
    pub trade_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayRecord {
    pub accrued_interest: String,
    pub created_at: i64,
    pub currency: String,
    pub daily_int_rate: String,
    pub liability: String,
    pub maturity_time: i64,
    pub principal: String,
    pub rapaid_size: Option<String>,
    pub term: i32,
    pub trade_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepaymentRecord {
    pub currency: String,
    pub daily_int_rate: String,
    pub interest: String,
    pub principal: String,
    pub rapaid_size: String,
    pub repay_time: String,
    pub term: i32,
    pub trade_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderId {
    pub order_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrder {
    pub order_id: String,
    pub currency: String,
    pub size: String,
    pub filled_size: String,
    pub daily_int_rate: String,
    pub term: i32,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginHistory {
    pub order_id: String,
    pub currency: String,
    pub size: String,
    pub filled_state: Option<String>,
    pub daily_int_rate: String,
    pub term: i32,
    pub created_at: i64,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendOrder {
    pub trade_id: String,
    pub currency: String,
    pub size: String,
    pub accrued_interest: String,
    pub repaid: String,
    pub daily_int_rate: String,
    pub term: i32,
    pub maturity_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendHistory {
    pub trade_id: String,
    pub currency: String,
    pub size: String,
    pub interest: String,
    pub repaid: String,
    pub daily_int_rate: String,
    pub term: i32,
    pub settled_at: i64,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendRecord {
    pub currency: String,
    pub outstanding: String,
    pub filled_size: String,
    pub accrued_interest: String,
    pub realized_profit: String,
    pub is_auto_lend: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendMarketData {
    pub daily_int_rate: String,
    pub term: i32,
    pub size: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeData {
    pub trade_id: String,
    pub currency: String,
    pub size: String,
    pub daily_int_rate: String,
    pub term: i32,
    pub timestamp: i64,
}
