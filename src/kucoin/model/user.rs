#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: String,
    pub sub_name: String,
    pub remarks: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountId {
    pub id: String,
}

pub enum AccountType {
    Main,
    Trade,
    Margin,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
    pub id: String,
    pub currency: String,
    pub r#type: String,
    pub balance: String,
    pub available: String,
    pub holds: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAccount {
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub holds: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub currency: String,
    pub amount: String,
    pub fee: String,
    pub balance: String,
    pub biz_type: String,
    pub direction: String,
    pub created_at: i64,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolds {
    pub currency: String,
    pub hold_amount: String,
    pub biz_type: String,
    pub order_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountBalances {
    pub sub_user_id: String,
    pub sub_name: String,
    pub main_accounts: Vec<SubAccountInfo>,
    pub trade_accounts: Vec<SubAccountInfo>,
    pub margin_accounts: Vec<SubAccountInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountInfo {
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub holds: String,
    pub base_currency: String,
    pub base_currency_price: String,
    pub base_amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableBalance {
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub holds: String,
    pub transferable: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderId {
    pub order_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
    pub memo: String,
    pub chain: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositList {
    pub address: String,
    pub memo: String,
    pub amount: i32,
    pub fee: f32,
    pub currency: String,
    pub is_inner: bool,
    pub wallet_tx_id: String,
    pub status: String,
    pub remark: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositListV1 {
    pub currency: String,
    pub created_at: i64,
    pub amount: String,
    pub wallet_tx_id: String,
    pub is_inner: bool,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalList {
    pub id: String,
    pub address: String,
    pub memo: String,
    pub currency: String,
    pub amount: f32,
    pub fee: f32,
    pub wallet_tx_id: String,
    pub is_inner: bool,
    pub status: String,
    pub remark: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalListV1 {
    pub currency: String,
    pub created_at: i64,
    pub amount: String,
    pub address: String,
    pub wallet_tx_id: String,
    pub is_inner: bool,
    pub status: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalQuotas {
    pub currency: String,
    pub limit_BTC_amount: String,
    pub used_BTC_amount: String,
    pub limit_amount: String,
    pub remain_amount: String,
    pub available_amount: String,
    pub withdrawal_min_fee: String,
    pub inner_withdraw_min_fee: String,
    pub withdraw_min_size: String,
    pub is_withdraw_enabled: String,
    pub precision: i32,
    pub chain: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalId {
    withdrawal_id: String,
}
