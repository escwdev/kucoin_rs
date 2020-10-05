use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServers {
    pub instance_servers: Vec<InstanceServer>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceServer {
    pub ping_interval: i32,
    pub endpoint: String,
    pub protocol: String,
    pub encrypt: bool,
    pub ping_timeout: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WSTopic {
    Ticker(Vec<String>),
    AllTicker,
    Snapshot(String),
    OrderBook(Vec<String>),
    Match(Vec<String>),
    FullMatch(Vec<String>),
    Level3Public(Vec<String>),
    Level3Private(Vec<String>),
    IndexPrice(Vec<String>),
    MarketPrice(Vec<String>),
    OrderBookChange(Vec<String>),
    StopOrder(Vec<String>),
    Balances,
    DebtRatio,
    PositionChange,
    MarginTradeOrder(String),
    TradeOrders
}

pub enum WSType {
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum KucoinWebsocketMsg {
    WelcomeMsg(DefaultMsg),
    SubscribeMsg(Subscribe),
    PingMsg(DefaultMsg),
    PongMsg(DefaultMsg),
    Ping,
    Pong,
    Binary(Vec<u8>),
    TickerMsg(WSResp<SymbolTicker>),
    AllTickerMsg(WSResp<SymbolTicker>),
    SnapshotMsg(WSResp<Snapshot>),
    OrderBookMsg(WSResp<Level2>),
    MatchMsg(WSResp<Match>),
    Level3ReceivedMsg(WSResp<Level3Received>),
    Level3OpenMsg(WSResp<Level3Open>),
    Level3MatchMsg(WSResp<Level3Match>),
    Level3DoneMsg(WSResp<Level3Done>),
    Level3ChangeMsg(WSResp<Level3Change>),
    FullMatchReceivedMsg(WSResp<FullMatchReceived>),
    FullMatchOpenMsg(WSResp<FullMatchOpen>),
    FullMatchDoneMsg(WSResp<FullMatchDone>),
    FullMatchMatchMsg(WSResp<FullMatchMatch>),
    FullMatchChangeMsg(WSResp<FullMatchChange>),
    IndexPriceMsg(WSResp<IndexPrice>),
    MarketPriceMsg(WSResp<MarketPrice>),
    OrderBookChangeMsg(WSResp<BookChange>),
    StopOrderMsg(WSResp<StopOrder>),
    BalancesMsg(WSResp<Balances>),
    DebtRatioMsg(WSResp<DebtRatio>),
    PositionChangeMsg(WSResp<PositionChange>),
    MarginTradeOpenMsg(WSResp<MarginTradeOpen>),
    MarginTradeUpdateMsg(WSResp<MarginTradeUpdate>),
    MarginTradeDoneMsg(WSResp<MarginTradeDone>),
    TradeOpenMsg(WSResp<TradeOpen>),
    TradeMatchMsg(WSResp<TradeMatch>),
    TradeFilledMsg(WSResp<TradeFilled>),
    TradeCanceledMsg(WSResp<TradeCanceled>),
    TradeUpdateMsg(WSResp<TradeUpdate>),
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResp<T> {
    pub r#type: String,
    pub topic: String,
    pub subject: String,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultMsg {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    pub id: String,
    pub r#type: String,
    pub topic: String,
    pub private_channel: bool,
    pub response: bool,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolTicker {
    pub sequence: String,
    pub best_ask: String,
    pub size: String,
    pub best_bid_size: String,
    pub price: String,
    pub best_ask_size: String,
    pub best_bid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub sequence: i64,
    pub data: SnapshotData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotData {
    pub trading: bool,
    pub symbol: String,
    pub buy: f32,
    pub sell: f32,
    pub sort: i32,
    pub vol_value: f32,
    pub base_currency: String,
    pub market: String,
    pub quote_currency: String,
    pub symbol_code: String,
    pub datetime: i64,
    pub high: Option<f32>,
    pub vol: f32,
    pub low: Option<f32>,
    pub change_price: Option<f32>,
    pub change_rate: f32,
    pub last_traded_price: f32,
    pub board: i32,
    pub mark: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2 {
    pub sequence_start: i64,
    pub sequence_end: i64,
    pub symbol: String,
    pub changes: Level2Changes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level2Changes {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Received {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: Option<String>,
    pub time: String,
    pub client_oid: Option<String>,
    pub r#type: String,
    pub order_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Open {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub order_id: String,
    pub price: String,
    pub time: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Done {
    pub sequence: String,
    pub symbol: String,
    pub reason: String,
    pub side: String,
    pub order_id: String,
    pub time: String,
    pub r#type: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Match {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub taker_order_id: String,
    pub time: String,
    pub r#type: String,
    pub maker_order_id: String,
    pub trade_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level3Change {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub price: String,
    pub new_size: String,
    pub time: String,
    pub r#type: String,
    pub old_size: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchReceived {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub client_oid: Option<String>,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchOpen {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub order_time: i64,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchDone {
    pub sequence: i64,
    pub symbol: String,
    pub order_id: String,
    pub reason: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchMatch {
    pub sequence: i64,
    pub symbol: String,
    pub side: String,
    pub price: String,
    pub remain_size: String,
    pub taker_order_id: String,
    pub maker_order_id: String,
    pub trade_id: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchChange {
    pub sequence: i64,
    pub symbol: String,
    pub size: String,
    pub order_id: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketPrice {
    pub symbol: String,
    pub granularity: i32,
    pub timestamp: i64,
    pub value: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookChange {
    pub sequence: i32,
    pub currency: String,
    pub daily_int_rate: f32,
    pub annual_int_rate: f32,
    pub term: i32,
    pub size: f32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrder {
    pub sequence: String,
    pub symbol: String,
    pub side: String,
    pub order_id: String,
    pub stop_entry: String,
    pub funds: String,
    pub time: String,
    pub r#type: String,
    pub reason: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub total: String,
    pub available: String,
    pub available_change: String,
    pub currency: String,
    pub hold: String,
    pub hold_change: String,
    pub relation_event: String,
    pub relation_event_id: String,
    pub time: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebtRatio {
    pub debt_ratio: f32,
    pub total_debt: String,
    pub debt_list: HashMap<String, String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionChange {
    pub r#type: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeOpen {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeUpdate {
    pub currency: String,
    pub order_id: String,
    pub daily_int_rate: f32,
    pub term: i32,
    pub size: i32,
    pub lent_size: f32,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTradeDone {
    pub currency: String,
    pub order_id: String,
    pub reason: String,
    pub side: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOpen {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    pub price: String,
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeMatch {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub liquidity: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    pub price: String,
    pub match_price: String,
    pub match_size: String,
    pub trade_id: String,
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeFilled {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    pub price: String,
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeCanceled {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    pub price: String,
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeUpdate {
    pub symbol: String,
    pub order_type: String,
    pub side: String,
    pub r#type: String,
    pub old_size: String,
    pub order_id: String,
    pub order_time: i64,
    pub size: String,
    pub filled_size: String,
    pub price: String,
    pub client_oid: String,
    pub remain_size: String,
    pub status: String,
    pub ts: i64,
}