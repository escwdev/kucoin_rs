#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolList {
    pub symbol: String,
    pub name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub quote_max_size: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub price_increment: String,
    pub fee_currency: String,
    pub enable_trading: bool,
    pub is_margin_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub sequence: String,
    pub best_ask: String,
    pub size: String,
    pub price: String,
    pub best_bid_size: String,
    pub best_bid: String,
    pub best_ask_size: String,
    pub time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllTickers {
    pub time: i64,
    pub ticker: Vec<Tick>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    pub symbol: String,
    pub symbol_name: String,
    pub buy: String,
    pub sell: String,
    pub change_rate: Option<String>,
    pub change_price: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub vol: String,
    pub vol_value: String,
    pub last: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStats {
    pub symbol: String,
    pub buy: String,
    pub sell: String,
    pub change_rate: Option<String>,
    pub change_price: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub vol: String,
    pub vol_value: String,
    pub last: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub sequence: i64,
    pub time: i64,
    pub bids: Vec<(String, String, String, i64)>,
    pub asks: Vec<(String, String, String, i64)>,
}

pub enum OrderBookType {
    L20,
    L100,
    Full,
    Level3,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistories {
    pub sequence: String,
    pub price: String,
    pub size: String,
    pub side: String,
    pub time: i64,
}

pub enum Klines {
    K1min,
    K3min,
    K5min,
    K15min,
    K30min,
    K1hour,
    K2hour,
    K4hour,
    K6hour,
    K8hour,
    K12hour,
    K1day,
    K1week,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    currency: String,
    name: String,
    full_name: String,
    precision: i32,
    withdrawal_min_size: String,
    withdrawal_min_fee: String,
    is_withdrawal_enabled: Option<bool>,
    is_deposit_enabled: bool,
    is_margin_enabled: bool,
    is_debit_enabled: bool,
}

pub enum Chain {
    OMNI,
    ERC20,
    TRC20,
}

pub enum Fiat {
    USD,
    EUR,
    CAD,
    CNY,
    AUD,
    KRW,
    JPY,
    GBP,
    INR,
    IDR,
    RUB,
    BRL,
    TRY,
    PLN,
    PHP,
    ZAR,
    THB,
    CHF,
    MYR,
    MXR,
    HRK,
    ARS,
    KZT,
    IRR,
    VND,
    ILS,
    BDT,
    HKD,
    TWD,
    COP,
    DKK,
    BGN,
    NOK,
    DZD,
    RON,
    SGD,
    NGN,
    CZK,
    PKR,
    SEK,
    NZD,
    UAH,
}
