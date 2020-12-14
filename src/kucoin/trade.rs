use reqwest::header;
use std::collections::HashMap;

use super::client::Kucoin;
use super::error::APIError;
use super::model::trade::{
    CancelByClientOidResp, CancelResp, FillsInfo, HistoricalOrder, OrderInfo, OrderResp,
};
use super::model::{APIData, APIDatum, Method, Pagination};
use super::utils::format_query;

impl Kucoin {
    /// Places a limit order. Takes required inputs directly and a Some<OrderOptionals> type, or None for
    /// optional inputs. See OrderOptionals for build pattern usage to simplify generating optional params.
    pub async fn post_limit_order(
        &self,
        client_oid: &str,
        symbol: &str,
        side: &str,
        price: f32,
        size: f32,
        optionals: Option<OrderOptionals<'_>>,
    ) -> Result<APIDatum<OrderResp>, APIError> {
        let endpoint = String::from("/api/v1/orders");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("clientOid"), client_oid.to_string());
        params.insert(String::from("symbol"), symbol.to_string());
        params.insert(String::from("side"), side.to_string());
        params.insert(String::from("price"), price.to_string());
        params.insert(String::from("size"), size.to_string());
        if let Some(opt) = optionals {
            let opts = parse_order(opt);
            params.extend(opts);
        };
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(headers), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Places a market order. Takes required inputs directly and a Some<OrderOptionals> type, or None for
    /// optional inputs. See OrderOptionals for build pattern usage to simplify generating optional params.
    ///
    /// Note that size is the amount in the base currency and funds is the amount in quote currency. Users
    /// should only use one or the other the order will fail. One of the two is a required parameter.
    pub async fn post_market_order(
        &self,
        client_oid: &str,
        symbol: &str,
        side: &str,
        size: Option<f32>,
        funds: Option<f32>,
        optionals: Option<OrderOptionals<'_>>,
    ) -> Result<APIDatum<OrderResp>, APIError> {
        let endpoint = String::from("/api/v1/orders");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("clientOid"), client_oid.to_string());
        params.insert(String::from("symbol"), symbol.to_string());
        params.insert(String::from("side"), side.to_string());
        params.insert(String::from("type"), String::from("market"));
        if let Some(s) = size {
            params.insert(String::from("size"), s.to_string());
        };
        if let Some(f) = funds {
            params.insert(String::from("funds"), f.to_string());
        };
        if let Some(opt) = optionals {
            let opts = parse_order(opt);
            params.extend(opts);
        };
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(headers), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// Cancels an order based on the provided order id (required).
    pub async fn cancel_order(&self, order_id: &str) -> Result<APIDatum<CancelResp>, APIError> {
        let endpoint = format!("/api/v1/orders/{}", order_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, None, None, Method::DELETE)
            .unwrap();
        let resp = self.delete(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    /// Cancels an order based on the provided order id (required).
    pub async fn cancel_order_by_client_oid(
        &self,
        client_oid: &str,
    ) -> Result<APIDatum<CancelByClientOidResp>, APIError> {
        let endpoint = format!("/api/v1/order/client-order/{}", client_oid);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, None, None, Method::DELETE)
            .unwrap();
        let resp = self.delete(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    // Cancels all orders of a given symbol (optional) or trade type (optional).
    pub async fn cancel_all_orders(
        &self,
        symbol: Option<&str>,
        trade_type: Option<&str>,
    ) -> Result<APIDatum<CancelResp>, APIError> {
        let endpoint = String::from("/api/v1/orders");
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(s) = symbol {
            params.insert(String::from("symbol"), s.to_owned());
        };
        if let Some(t) = trade_type {
            params.insert(String::from("tradeType"), t.to_owned());
        };
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
            headers = self
                .sign_headers(endpoint, Some(&params), None, Method::DELETE)
                .unwrap();
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
            headers = self
                .sign_headers(endpoint, None, None, Method::DELETE)
                .unwrap();
        };
        let resp = self.delete(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    // Consider list orders
    pub async fn get_orders(
        &self,
        optionals: Option<OrderInfoOptionals<'_>>,
    ) -> Result<APIDatum<Pagination<OrderInfo>>, APIError> {
        let endpoint = String::from("/api/v1/orders");
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(opts) = optionals {
            if let Some(o) = opts.status {
                params.insert("status".to_string(), o.to_string());
            };
            if let Some(o) = opts.symbol {
                params.insert("symbol".to_string(), o.to_string());
            };
            if let Some(o) = opts.side {
                params.insert("side".to_string(), o.to_string());
            };
            if let Some(o) = opts.r#type {
                params.insert("type".to_string(), o.to_string());
            };
            if let Some(o) = opts.trade_type {
                params.insert("tradeType".to_string(), o.to_string());
            };
            if let Some(o) = opts.start_at {
                params.insert("startAt".to_string(), o.to_string());
            };
            if let Some(o) = opts.end_at {
                params.insert("endAt".to_string(), o.to_string());
            };
            if let Some(o) = opts.current_page {
                params.insert("currentPage".to_string(), o.to_string());
            };
            if let Some(o) = opts.page_size {
                params.insert("pageSize".to_string(), o.to_string());
            };
        };
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
            headers = self
                .sign_headers(endpoint, None, Some(query), Method::GET)
                .unwrap();
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
            headers = self
                .sign_headers(endpoint, None, None, Method::GET)
                .unwrap();
        }
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_v1_historical_orders(
        &self,
        symbol: Option<&str>,
        start_at: Option<i64>,
        end_at: Option<i64>,
        side: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<HistoricalOrder>>, APIError> {
        let endpoint = String::from("/api/v1/orders");
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(o) = current_page {
            params.insert("current_page".to_string(), o.to_string());
        };
        if let Some(o) = page_size {
            params.insert("page_size".to_string(), o.to_string());
        };
        if let Some(o) = symbol {
            params.insert("symbol".to_string(), o.to_string());
        };
        if let Some(o) = start_at {
            params.insert("start_at".to_string(), o.to_string());
        };
        if let Some(o) = end_at {
            params.insert("end_at".to_string(), o.to_string());
        };
        if let Some(o) = side {
            params.insert("side".to_string(), o.to_string());
        };
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
            headers = self
                .sign_headers(endpoint, None, Some(query), Method::GET)
                .unwrap();
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
            headers = self
                .sign_headers(endpoint, None, None, Method::GET)
                .unwrap();
        }
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_recent_orders(&self) -> Result<APIData<OrderInfo>, APIError> {
        let endpoint = String::from("/api/v1/limit/orders");
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_order(&self, order_id: &str) -> Result<APIDatum<OrderInfo>, APIError> {
        let endpoint = format!("/api/v1/orders/{}", order_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers: header::HeaderMap = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_fills(
        &self,
        optionals: Option<FillsOptionals<'_>>,
    ) -> Result<APIDatum<Pagination<FillsInfo>>, APIError> {
        let endpoint = String::from("/api/v1/fills");
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(opts) = optionals {
            if let Some(o) = opts.order_id {
                params.insert("order_id".to_string(), o.to_string());
            };
            if let Some(o) = opts.symbol {
                params.insert("symbol".to_string(), o.to_string());
            };
            if let Some(o) = opts.side {
                params.insert("side".to_string(), o.to_string());
            };
            if let Some(o) = opts.r#type {
                params.insert("type".to_string(), o.to_string());
            };
            if let Some(o) = opts.start_at {
                params.insert("startAt".to_string(), o.to_string());
            };
            if let Some(o) = opts.end_at {
                params.insert("endAt".to_string(), o.to_string());
            };
            if let Some(o) = opts.trade_type {
                params.insert("tradeType".to_string(), o.to_string());
            };
            if let Some(o) = opts.current_page {
                params.insert("currentPage".to_string(), o.to_string());
            };
            if let Some(o) = opts.page_size {
                params.insert("pageSize".to_string(), o.to_string());
            };
        };
        if !params.is_empty() {
            let query = format_query(&params);
            url = format!("{}{}{}", &self.prefix, endpoint, query);
            headers = self
                .sign_headers(endpoint, None, Some(query), Method::GET)
                .unwrap();
        } else {
            url = format!("{}{}", &self.prefix, endpoint);
            headers = self
                .sign_headers(endpoint, None, None, Method::GET)
                .unwrap();
        };
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_recent_fills(&self) -> Result<APIData<FillsInfo>, APIError> {
        let endpoint = String::from("/api/v1/limit/fills");
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }
}

fn parse_order(optionals: OrderOptionals) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();

    if let Some(o) = optionals.remark {
        params.insert(String::from("remark"), o.to_string());
    };
    if let Some(o) = optionals.stop {
        params.insert(String::from("stop"), o.to_string());
    };
    if let Some(o) = optionals.stop_price {
        params.insert(String::from("stopPrice"), o.to_string());
    };
    if let Some(o) = optionals.time_in_force {
        params.insert(String::from("timeInForce"), o.to_string());
    };
    if let Some(o) = optionals.cancel_after {
        params.insert(String::from("cancelAfter"), o.to_string());
    };
    if let Some(o) = optionals.post_only {
        params.insert(String::from("postOnly"), o.to_string());
    };
    if let Some(o) = optionals.hidden {
        params.insert(String::from("hidden"), o.to_string());
    };
    if let Some(o) = optionals.iceberg {
        params.insert(String::from("iceberg"), o.to_string());
    };
    if let Some(o) = optionals.visible_size {
        params.insert(String::from("visibleSize"), o.to_string());
    };

    params
}

/// OrderOptionals contains a builder pattern that can be used to more easily take advantage of optional inputs.
///
/// Example:
/// ``` rust
/// use kucoin_rs::kucoin::trade::OrderOptionals;
///
///     let options = OrderOptionals::new()
///         .remark("Example of OrderOptionals builder pattern")
///         .stp("CO")
///         .hidden(true)
///         .build();
/// ```
///
/// See the Kucoin documentation for full list of options relative to market and limit orders.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrderOptionals<'a> {
    pub remark: Option<&'a str>,
    pub stop: Option<&'a str>,
    pub stop_price: Option<&'a str>,
    pub stp: Option<&'a str>,
    pub trade_type: Option<&'a str>,
    pub time_in_force: Option<&'a str>,
    pub cancel_after: Option<i64>,
    pub post_only: Option<bool>,
    pub hidden: Option<bool>,
    pub iceberg: Option<bool>,
    pub visible_size: Option<&'a str>,
}

#[allow(dead_code)]
/// Generates new empty OrderOptionals struct ready for building.
impl<'a> OrderOptionals<'a> {
    pub fn new() -> Self {
        OrderOptionals {
            remark: None,
            stop: None,
            stop_price: None,
            stp: None,
            time_in_force: None,
            trade_type: None,
            cancel_after: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
        }
    }

    pub fn remark(&mut self, r: &'a str) -> &mut Self {
        self.remark = Some(r);
        self
    }

    pub fn stop(&mut self, s: &'a str) -> &mut Self {
        self.stop = Some(s);
        self
    }

    pub fn stop_price(&mut self, s: &'a str) -> &mut Self {
        self.stop_price = Some(s);
        self
    }

    pub fn stp(&mut self, s: &'a str) -> &mut Self {
        self.stp = Some(s);
        self
    }

    pub fn time_in_force(&mut self, t: &'a str) -> &mut Self {
        self.time_in_force = Some(t);
        self
    }

    pub fn trade_type(&mut self, t: &'a str) -> &mut Self {
        self.trade_type = Some(t);
        self
    }

    pub fn cancel_after(&mut self, c: i64) -> &mut Self {
        self.cancel_after = Some(c);
        self
    }

    pub fn post_only(&mut self, p: bool) -> &mut Self {
        self.post_only = Some(p);
        self
    }

    pub fn hidden(&mut self, h: bool) -> &mut Self {
        self.hidden = Some(h);
        self
    }

    pub fn iceberg(&mut self, i: bool) -> &mut Self {
        self.iceberg = Some(i);
        self
    }

    pub fn visible_size(&mut self, v: &'a str) -> &mut Self {
        self.visible_size = Some(v);
        self
    }

    /// Builds an OrderOptional Type from chained optional funtions
    /// to be used with posting orders. Only contains optional inputs
    /// the post order functions require specific required inputs.
    /// See those functions' documentation for details.
    pub fn build(&self) -> Self {
        Self {
            remark: self.remark,
            stop: self.stop,
            stop_price: self.stop_price,
            stp: self.stp,
            time_in_force: self.time_in_force,
            trade_type: self.trade_type,
            cancel_after: self.cancel_after,
            post_only: self.post_only,
            hidden: self.hidden,
            iceberg: self.iceberg,
            visible_size: self.visible_size,
        }
    }
}

/// OrderInfoOptionals contains a builder pattern that can be used to more easily take advantage of optional inputs.
///
/// Example:
/// ``` rust
/// use kucoin_rs::kucoin::trade::OrderInfoOptionals;
///
///     let options = OrderInfoOptionals::new()
///         .symbol("BTC-USDT")
///         .side("buy")
///         .build();
/// ```
///
/// See the Kucoin documentation for full list of options relative to market and limit orders.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrderInfoOptionals<'a> {
    pub status: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub side: Option<&'a str>,
    pub r#type: Option<&'a str>,
    pub trade_type: Option<&'a str>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    pub current_page: Option<i32>,
    pub page_size: Option<i32>,
}

impl<'a> OrderInfoOptionals<'a> {
    pub fn new() -> Self {
        OrderInfoOptionals {
            status: None,
            symbol: None,
            side: None,
            r#type: None,
            trade_type: None,
            start_at: None,
            end_at: None,
            current_page: None,
            page_size: None,
        }
    }

    pub fn status(&mut self, s: &'a str) -> &mut Self {
        self.status = Some(s);
        self
    }

    pub fn symbol(&mut self, s: &'a str) -> &mut Self {
        self.symbol = Some(s);
        self
    }

    pub fn side(&mut self, s: &'a str) -> &mut Self {
        self.side = Some(s);
        self
    }

    pub fn order_type(&mut self, s: &'a str) -> &mut Self {
        self.r#type = Some(s);
        self
    }

    pub fn trade_type(&mut self, s: &'a str) -> &mut Self {
        self.trade_type = Some(s);
        self
    }

    pub fn start_at(&mut self, i: i64) -> &mut Self {
        self.start_at = Some(i);
        self
    }

    pub fn end_at(&mut self, i: i64) -> &mut Self {
        self.end_at = Some(i);
        self
    }

    pub fn current_page(&mut self, i: i32) -> &mut Self {
        self.current_page = Some(i);
        self
    }

    pub fn page_size(&mut self, i: i32) -> &mut Self {
        self.page_size = Some(i);
        self
    }

    /// Builds an OrderInfoOptional Type from chained optional funtions
    /// to be used with getting lists of orders. Only contains optional inputs
    /// the post order functions require specific required inputs.
    /// See the function's documentation for details.
    pub fn build(&self) -> Self {
        OrderInfoOptionals {
            status: self.status,
            symbol: self.symbol,
            side: self.side,
            r#type: self.r#type,
            trade_type: self.trade_type,
            start_at: self.start_at,
            end_at: self.end_at,
            current_page: self.current_page,
            page_size: self.page_size,
        }
    }
}

/// FillsOptionals contains a builder pattern that can be used to more easily take advantage of optional inputs.
///
/// Example:
/// ``` rust
/// use kucoin_rs::kucoin::trade::FillsOptionals;
///     let options = FillsOptionals::new()
///         .symbol("BTC-USDT")
///         .side("buy")
///         .build();
/// ```
///
/// See the Kucoin documentation for full list of options relative to market and limit orders.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FillsOptionals<'a> {
    pub order_id: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub side: Option<&'a str>,
    pub r#type: Option<&'a str>,
    pub start_at: Option<i64>,
    pub end_at: Option<i64>,
    pub trade_type: Option<&'a str>,
    pub current_page: Option<i32>,
    pub page_size: Option<i32>,
}

impl<'a> FillsOptionals<'a> {
    pub fn new() -> Self {
        FillsOptionals {
            order_id: None,
            symbol: None,
            side: None,
            r#type: None,
            start_at: None,
            end_at: None,
            trade_type: None,
            current_page: None,
            page_size: None,
        }
    }

    pub fn order_id(&mut self, s: &'a str) -> &mut Self {
        self.order_id = Some(s);
        self
    }

    pub fn symbol(&mut self, s: &'a str) -> &mut Self {
        self.symbol = Some(s);
        self
    }

    pub fn side(&mut self, s: &'a str) -> &mut Self {
        self.side = Some(s);
        self
    }

    pub fn order_type(&mut self, s: &'a str) -> &mut Self {
        self.r#type = Some(s);
        self
    }

    pub fn trade_type(&mut self, s: &'a str) -> &mut Self {
        self.trade_type = Some(s);
        self
    }

    pub fn start_at(&mut self, i: i64) -> &mut Self {
        self.start_at = Some(i);
        self
    }

    pub fn end_at(&mut self, i: i64) -> &mut Self {
        self.end_at = Some(i);
        self
    }

    pub fn current_page(&mut self, i: i32) -> &mut Self {
        self.current_page = Some(i);
        self
    }

    pub fn page_size(&mut self, i: i32) -> &mut Self {
        self.page_size = Some(i);
        self
    }

    /// Builds an FillsOptional Type from chained optional funtions
    /// to be used with getting lists of fills. Only contains optional inputs
    /// the post order functions require specific required inputs.
    /// See the function's documentation for details.
    pub fn build(&self) -> Self {
        FillsOptionals {
            order_id: self.order_id,
            symbol: self.symbol,
            side: self.side,
            r#type: self.r#type,
            trade_type: self.trade_type,
            start_at: self.start_at,
            end_at: self.end_at,
            current_page: self.current_page,
            page_size: self.page_size,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::kucoin::trade::{FillsOptionals, OrderInfoOptionals, OrderOptionals};
    #[test]
    fn use_build_pattern_all_order_optionals() {
        let options = OrderOptionals {
            remark: Some("Test build pattern"),
            stop: Some("loss"),
            stop_price: Some("12.321"),
            stp: Some("CO"),
            time_in_force: Some("GTT"),
            trade_type: Some("TRADE"),
            cancel_after: Some(1_231_231_321_321),
            post_only: Some(true),
            hidden: Some(true),
            iceberg: Some(false),
            visible_size: Some("1.23"),
        };

        let builder_options = OrderOptionals::new()
            .remark("Test build pattern")
            .stop("loss")
            .stop_price("12.321")
            .stp("CO")
            .time_in_force("GTT")
            .trade_type("TRADE")
            .cancel_after(1_231_231_321_321)
            .post_only(true)
            .hidden(true)
            .iceberg(false)
            .visible_size("1.23")
            .build();

        assert_eq!(builder_options, options)
    }

    #[test]
    fn use_build_pattern_some_order_optionals() {
        let options = OrderOptionals {
            remark: Some("Test build pattern"),
            stop: None,
            stop_price: None,
            stp: Some("CO"),
            time_in_force: Some("GTT"),
            trade_type: Some("TRADE"),
            cancel_after: Some(1_231_231_321_321),
            post_only: Some(true),
            hidden: None,
            iceberg: None,
            visible_size: None,
        };

        let builder_options = OrderOptionals::new()
            .remark("Test build pattern")
            .stp("CO")
            .time_in_force("GTT")
            .trade_type("TRADE")
            .cancel_after(1_231_231_321_321)
            .post_only(true)
            .build();

        assert_eq!(builder_options, options)
    }

    #[test]
    fn use_build_pattern_all_order_info_optionals() {
        let options = OrderInfoOptionals {
            status: Some("active"),
            symbol: Some("BTC-USDT"),
            side: Some("buy"),
            r#type: Some("limit"),
            trade_type: Some("TRADE"),
            start_at: Some(1_580_683_419_725),
            end_at: Some(1_580_683_800_000),
            current_page: Some(1),
            page_size: Some(50),
        };

        let build_options = OrderInfoOptionals::new()
            .status("active")
            .symbol("BTC-USDT")
            .side("buy")
            .order_type("limit")
            .trade_type("TRADE")
            .start_at(1_580_683_419_725)
            .end_at(1_580_683_800_000)
            .current_page(1)
            .page_size(50)
            .build();

        assert_eq!(options, build_options)
    }

    #[test]
    fn use_build_pattern_some_order_info_optionals() {
        let options = OrderInfoOptionals {
            status: None,
            symbol: Some("BTC-USDT"),
            side: None,
            r#type: None,
            trade_type: None,
            start_at: Some(1_580_683_419_725),
            end_at: Some(1_580_683_800_000),
            current_page: None,
            page_size: None,
        };

        let build_options = OrderInfoOptionals::new()
            .symbol("BTC-USDT")
            .start_at(1_580_683_419_725)
            .end_at(1_580_683_800_000)
            .build();

        assert_eq!(options, build_options)
    }

    #[test]
    fn use_build_pattern_all_fills_optionals() {
        let options = FillsOptionals {
            order_id: Some("asdasd-sadasda-asxsaxs"),
            symbol: Some("BTC-USDT"),
            side: Some("buy"),
            r#type: Some("limit"),
            trade_type: Some("TRADE"),
            start_at: Some(1_580_683_419_725),
            end_at: Some(1_580_683_800_000),
            current_page: Some(1),
            page_size: Some(50),
        };

        let build_options = FillsOptionals::new()
            .order_id("asdasd-sadasda-asxsaxs")
            .symbol("BTC-USDT")
            .side("buy")
            .order_type("limit")
            .trade_type("TRADE")
            .start_at(1_580_683_419_725)
            .end_at(1_580_683_800_000)
            .current_page(1)
            .page_size(50)
            .build();

        assert_eq!(options, build_options)
    }

    #[test]
    fn use_build_pattern_some_fills_optionals() {
        let options = FillsOptionals {
            order_id: None,
            symbol: Some("BTC-USDT"),
            side: None,
            r#type: None,
            trade_type: None,
            start_at: Some(1_580_683_419_725),
            end_at: Some(1_580_683_800_000),
            current_page: None,
            page_size: None,
        };

        let build_options = FillsOptionals::new()
            .symbol("BTC-USDT")
            .start_at(1_580_683_419_725)
            .end_at(1_580_683_800_000)
            .build();

        assert_eq!(options, build_options)
    }
}
