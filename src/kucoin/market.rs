use std::collections::HashMap;

use reqwest::header;

use super::client::Kucoin;
use super::error::APIError;
use super::model::market::{
    AllTickers, Chain, Currency, DailyStats, Klines, OrderBook, AtomicOrderBook, OrderBookType, SymbolList, Ticker,
    TradeHistories
};
use super::model::{APIData, APIDatum, Method};
use super::utils::format_query;

impl Kucoin {
    pub async fn get_symbol_list(
        &self,
        market: Option<&str>,
    ) -> Result<APIData<SymbolList>, APIError> {
        let endpoint = String::from("/api/v1/symbols");
        let url = match market {
            Some(m) => format!("{}{}?market={}", &self.prefix, endpoint, m),
            None => format!("{}{}", &self.prefix, endpoint),
        };
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<APIDatum<Ticker>, APIError> {
        let endpoint = String::from("/api/v1/market/orderbook/level1");
        let url = format!("{}{}?symbol={}", &self.prefix, endpoint, symbol);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_all_tickers(&self) -> Result<APIDatum<AllTickers>, APIError> {
        let endpoint = String::from("/api/v1/market/allTickers");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_daily_stats(&self, symbol: &str) -> Result<APIDatum<DailyStats>, APIError> {
        let endpoint = String::from("/api/v1/market/stats");
        let url = format!("{}{}?symbol={}", &self.prefix, endpoint, symbol);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_market_list(&self) -> Result<APIData<String>, APIError> {
        let endpoint = String::from("/api/v1/markets");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_orderbook(
        &self,
        symbol: &str,
        amount: OrderBookType,
    ) -> Result<APIDatum<OrderBook>, APIError> {
        let endpoint = match amount {
            OrderBookType::L20 => format!("/api/v1/market/orderbook/level2_20?symbol={}", symbol),
            OrderBookType::L100 => format!("/api/v1/market/orderbook/level2_100?symbol={}", symbol),
            OrderBookType::Full => format!("/api/v3/market/orderbook/level2?symbol={}", symbol),
        };
        match amount {
            OrderBookType::L20 | OrderBookType::L100 => {
                let url = format!("{}{}", &self.prefix, endpoint);
                let resp: APIDatum<OrderBook> = self.get(url, None).await?.json().await?;
                return Ok(resp)
            },
            OrderBookType::Full => {
                let url = format!("{}{}", &self.prefix, endpoint);
                let headers: header::HeaderMap = self
                    .sign_headers(endpoint, None, None, Method::GET)
                    .unwrap();
                let resp = self.get(url, Some(headers)).await?.json().await?;
                return Ok(resp)
            },
        }
    }

    pub async fn get_atomic_orderbook(
        &self,
        symbol: &str,
    ) -> Result<APIDatum<AtomicOrderBook>, APIError> {
        let endpoint = format!("/api/v3/market/orderbook/level3?symbol={}", symbol);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_trade_histories(
        &self,
        symbol: &str,
    ) -> Result<APIData<TradeHistories>, APIError> {
        let endpoint = format!("/api/v1/market/histories?symbol={}", symbol);
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_klines(
        &self,
        klines: Klines,
        symbol: &str,
        start_at: Option<i64>,
        end_at: Option<i64>,
    ) -> Result<APIData<Vec<String>>, APIError> {
        let mut endpoint = String::from("/api/v1/market/candles?");
        match klines {
            Klines::K1min => endpoint.push_str("type=1min"),
            Klines::K3min => endpoint.push_str("type=3min"),
            Klines::K5min => endpoint.push_str("type=5min"),
            Klines::K15min => endpoint.push_str("type=15min"),
            Klines::K30min => endpoint.push_str("type=30min"),
            Klines::K1hour => endpoint.push_str("type=1hour"),
            Klines::K2hour => endpoint.push_str("type=2hour"),
            Klines::K4hour => endpoint.push_str("type=4hour"),
            Klines::K6hour => endpoint.push_str("type=6hour"),
            Klines::K8hour => endpoint.push_str("type=8hour"),
            Klines::K12hour => endpoint.push_str("type=12hour"),
            Klines::K1day => endpoint.push_str("type=1day"),
            Klines::K1week => endpoint.push_str("type=1week"),
        }
        endpoint.push_str(&format!("&symbol={}", symbol));
        if let Some(t) = start_at {
            endpoint.push_str(&format!("&startAt={}", t.to_string()));
        }
        if let Some(t) = end_at {
            endpoint.push_str(&format!("&endAt={}", t.to_string()));
        }
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_currencies(&self) -> Result<APIData<Currency>, APIError> {
        let endpoint = String::from("/api/v1/currencies");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_currency(
        &self,
        currency: &str,
        chain: Option<Chain>,
    ) -> Result<APIDatum<Currency>, APIError> {
        let mut endpoint = format!("/api/v1/currencies/{}", currency);
        if let Some(c) = chain {
            match c {
                Chain::OMNI => endpoint.push_str("?chain=OMNI"),
                Chain::ERC20 => endpoint.push_str("?chain=ERC20"),
                Chain::TRC20 => endpoint.push_str("?chain=TRC20"),
            }
        }
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_fiat_prices(
        &self,
        base: Option<&str>,
        currencies: Option<&str>,
    ) -> Result<APIDatum<HashMap<String, String>>, APIError> {
        let endpoint = String::from("/api/v1/prices");
        let mut params: HashMap<String, String> = HashMap::new();
        let url: String;
        if let Some(b) = base {
            params.insert(String::from("base"), b.to_string());
        }
        if let Some(c) = currencies {
            params.insert(String::from("currencies"), c.to_string());
        }
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
        }
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_server_time(&self) -> Result<APIDatum<i64>, APIError> {
        let endpoint = String::from("/api/v1/timestamp");
        let url = format!("{}{}", &self.prefix, endpoint);
        let resp = self.get(url, None).await?.json().await?;
        Ok(resp)
    }
}
