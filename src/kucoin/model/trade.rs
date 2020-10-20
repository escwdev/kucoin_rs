#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResp {
    pub order_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResp {
    pub cancelled_order_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByClientOidResp {
    pub cancelled_order_id: String,
    pub client_oid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub id: String,
    pub symbol: String,
    pub op_type: String,
    pub r#type: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub funds: String,
    pub deal_funds: String,
    pub deal_size: String,
    pub fee: String,
    pub fee_currency: String,
    pub stp: String,
    pub stop: String,
    pub stop_triggered: bool,
    pub stop_price: String,
    pub time_in_force: String,
    pub post_only: bool,
    pub hidden: bool,
    pub iceberg: bool,
    pub visible_size: String,
    pub cancel_after: i64,
    pub channel: String,
    pub client_oid: String,
    pub remark: Option<String>,
    pub tags: Option<String>,
    pub is_active: Option<bool>,
    pub cancel_exist: bool,
    pub created_at: i64,
    pub trade_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrder {
    symbol: String,
    deal_price: Option<String>,
    deal_value: Option<String>,
    amount: Option<String>,
    fee: String,
    side: String,
    created_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FillsInfo {
    pub symbol: String,
    pub trade_id: String,
    pub order_id: String,
    pub counter_order_id: String,
    pub side: String,
    pub liquidity: String,
    pub force_taker: bool,
    pub price: String,
    pub size: String,
    pub funds: String,
    pub fee: String,
    pub fee_rate: String,
    pub fee_currency: String,
    pub stop: String,
    pub r#type: String,
    pub created_at: i64,
    pub trade_type: String,
}
