use reqwest::header;
use std::collections::HashMap;

use super::client::Kucoin;
use super::error::APIError;
use super::model::user::{
    AccountHolds, AccountId, AccountInfo, AccountType, Accounts, DepositAddress, DepositList,
    DepositListV1, OrderId, SingleAccount, SubAccountBalances, TransferableBalance, UserInfo,
    WithdrawalId, WithdrawalList, WithdrawalListV1, WithdrawalQuotas,
};
use super::model::{APIData, APIDatum, Method, Pagination};
use super::utils::format_query;

impl Kucoin {
    pub async fn get_user_subaccount_info(&self) -> Result<APIData<UserInfo>, APIError> {
        let endpoint = String::from("/api/v1/sub/user");
        let url = format!("{}{}", &self.prefix, endpoint);
        let header = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(header)).await?.json().await?;
        Ok(resp)
    }

    pub async fn create_account(
        &self,
        account_type: AccountType,
        currency: &str,
    ) -> Result<APIDatum<AccountId>, APIError> {
        let endpoint = String::from("/api/v1/accounts");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        match account_type {
            AccountType::Main => params.insert(String::from("type"), String::from("main")),
            AccountType::Margin => params.insert(String::from("type"), String::from("margin")),
            AccountType::Trade => params.insert(String::from("type"), String::from("trade")),
        };
        params.insert(String::from("currency"), currency.to_string());
        let header = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(header), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn get_accounts_list(
        &self,
        currency: Option<&str>,
        acct_type: Option<&str>,
    ) -> Result<APIData<Accounts>, APIError> {
        let mut params: HashMap<String, String> = HashMap::new();
        let headers: header::HeaderMap;
        let url: String;
        let endpoint = String::from("/api/v1/accounts");
        if let Some(c) = currency {
            params.insert("currency".to_string(), c.to_owned());
        }
        if let Some(a) = acct_type {
            params.insert("type".to_string(), a.to_owned());
        }
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

    pub async fn get_account(&self, account_id: &str) -> Result<APIDatum<SingleAccount>, APIError> {
        let endpoint = format!("/api/v1/accounts/{}", account_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_account_ledgers(
        &self,
        account_id: &str,
        start_at: Option<i64>,
        end_at: Option<i64>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<AccountInfo>>, APIError> {
        let endpoint = format!("/api/v1/accounts/{}/ledgers", account_id);
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(t) = start_at {
            params.insert(String::from("startAt"), t.to_string());
        }
        if let Some(t) = end_at {
            params.insert(String::from("endAt"), t.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
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

    pub async fn get_hold(
        &self,
        account_id: &str,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<AccountHolds>>, APIError> {
        let endpoint = format!("/api/v1/accounts/{}/holds", account_id);
        let url: String;
        let headers: header::HeaderMap;
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
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

    pub async fn get_subaccount_balances(
        &self,
        account_id: &str,
    ) -> Result<APIDatum<SubAccountBalances>, APIError> {
        let endpoint = format!("/api/v1/sub-accounts/{}", account_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_all_subaccount_balances(
        &self,
    ) -> Result<APIData<SubAccountBalances>, APIError> {
        let endpoint = String::from("/api/v1/sub-accounts");
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_transferable_balance(
        &self,
        currency: &str,
        account_type: AccountType,
    ) -> Result<APIDatum<TransferableBalance>, APIError> {
        let mut endpoint = format! {"/api/v1/accounts/transferable?currency={}", currency};
        match account_type {
            AccountType::Main => endpoint.push_str("&type=MAIN"),
            AccountType::Margin => endpoint.push_str("&type=MARGIN"),
            AccountType::Trade => endpoint.push_str("&type=TRADE"),
        };
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?.json().await?;
        Ok(resp)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn transfer_to_subaccount(
        &self,
        client_oid: &str,
        currency: &str,
        amount: f32,
        direction: &str,
        sub_user_id: &str,
        account_type: Option<&str>,
        sub_account_type: Option<&str>,
    ) -> Result<APIDatum<OrderId>, APIError> {
        let endpoint = String::from("/api/v2/accounts/sub-transfer");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("clientOid"), client_oid.to_string());
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("amount"), amount.to_string());
        params.insert(String::from("direction"), direction.to_string());
        params.insert(String::from("subUserId"), sub_user_id.to_string());
        if let Some(a) = account_type {
            params.insert(String::from("accountType"), a.to_string());
        }
        if let Some(s) = sub_account_type {
            params.insert(String::from("subAccountType"), s.to_string());
        }
        let headers = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(headers), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn inner_transfer(
        &self,
        client_oid: &str,
        currency: &str,
        from: &str,
        to: &str,
        amount: &str,
    ) -> Result<APIDatum<OrderId>, APIError> {
        let endpoint = String::from("/api/v2/accounts/inner-transfer");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("clientOid"), client_oid.to_string());
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("from"), from.to_string());
        params.insert(String::from("to"), to.to_string());
        params.insert(String::from("amount"), amount.to_string());
        let headers = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(headers), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn create_deposit_address(
        &self,
        currency: &str,
        chain: Option<&str>,
    ) -> Result<APIDatum<DepositAddress>, APIError> {
        let endpoint = String::from("/api/v1/deposit-addresses");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        if let Some(c) = chain {
            params.insert(String::from("chain"), c.to_string());
        }
        let headers = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self
            .post(url, Some(headers), Some(params))
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn get_deposit_address(
        &self,
        currency: &str,
        chain: Option<&str>,
    ) -> Result<APIDatum<DepositAddress>, APIError> {
        let endpoint = String::from("/api/v2/deposit-addresses");
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        if let Some(c) = chain {
            params.insert(String::from("chain"), c.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_deposit_list(
        &self,
        currency: Option<&str>,
        start_at: Option<i64>,
        end_at: Option<i64>,
        status: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<DepositList>>, APIError> {
        let endpoint = String::from("/api/v1/deposits");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(t) = start_at {
            params.insert(String::from("startAt"), t.to_string());
        }
        if let Some(t) = end_at {
            params.insert(String::from("endAt"), t.to_string());
        }
        if let Some(s) = status {
            params.insert(String::from("status"), s.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_v1_deposit_list(
        &self,
        currency: Option<&str>,
        start_at: Option<i64>,
        end_at: Option<i64>,
        status: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<DepositListV1>>, APIError> {
        let endpoint = String::from("/api/v1/deposits");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(t) = start_at {
            params.insert(String::from("startAt"), t.to_string());
        }
        if let Some(t) = end_at {
            params.insert(String::from("endAt"), t.to_string());
        }
        if let Some(s) = status {
            params.insert(String::from("status"), s.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_withdrawals_list(
        &self,
        currency: Option<&str>,
        start_at: Option<i64>,
        end_at: Option<i64>,
        status: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<WithdrawalList>>, APIError> {
        let endpoint = String::from("/api/v1/withdrawals");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(t) = start_at {
            params.insert(String::from("startAt"), t.to_string());
        }
        if let Some(t) = end_at {
            params.insert(String::from("endAt"), t.to_string());
        }
        if let Some(s) = status {
            params.insert(String::from("status"), s.to_string());
        }
        if let Some(c) = current_page {
            params.insert(String::from("currentPage"), c.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_v1_withdrawals_list(
        &self,
        currency: Option<&str>,
        start_at: Option<i64>,
        end_at: Option<i64>,
        status: Option<&str>,
        current_page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<APIDatum<Pagination<WithdrawalListV1>>, APIError> {
        let endpoint = String::from("/api/v1/withdrawals");
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(c) = currency {
            params.insert(String::from("currency"), c.to_string());
        }
        if let Some(t) = start_at {
            params.insert(String::from("startAt"), t.to_string());
        }
        if let Some(t) = end_at {
            params.insert(String::from("endAt"), t.to_string());
        }
        if let Some(s) = status {
            params.insert(String::from("status"), s.to_string());
        }
        if let Some(p) = current_page {
            params.insert(String::from("currentPage"), p.to_string());
        }
        if let Some(p) = page_size {
            params.insert(String::from("pageSize"), p.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_withdrawals_quotas(
        &self,
        currency: &str,
        chain: Option<&str>,
    ) -> Result<APIDatum<WithdrawalQuotas>, APIError> {
        let endpoint = String::from("/api/v1/withdrawals/quotas");
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        if let Some(c) = chain {
            params.insert(String::from("chain"), c.to_string());
        }
        let query = format_query(&params);
        let url = format!("{}{}{}", &self.prefix, endpoint, query);
        let headers = self
            .sign_headers(endpoint, None, Some(query), Method::GET)
            .unwrap();
        let resp = self.get(url, Some(headers)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn apply_withdrawal(
        &self,
        currency: &str,
        address: &str,
        amount: i32,
        memo: Option<&str>,
        is_inner: Option<bool>,
        remark: Option<&str>,
        chain: Option<&str>,
    ) -> Result<APIDatum<WithdrawalId>, APIError> {
        let endpoint = String::from("/api/v1/withdrawals");
        let url = format!("{}{}", &self.prefix, endpoint);
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("currency"), currency.to_string());
        params.insert(String::from("address"), address.to_string());
        params.insert(String::from("amount"), amount.to_string());
        if let Some(m) = memo {
            params.insert(String::from("memo"), m.to_string());
        }
        if let Some(i) = is_inner {
            params.insert(String::from("isInner"), i.to_string());
        }
        if let Some(r) = remark {
            params.insert(String::from("remark"), r.to_string());
        }
        if let Some(c) = chain {
            params.insert(String::from("chain"), c.to_string());
        }
        let headers = self
            .sign_headers(endpoint, Some(&params), None, Method::POST)
            .unwrap();
        let resp = self.post(url, Some(headers), Some(params)).await?;
        let api_data = resp.json().await?;
        Ok(api_data)
    }

    pub async fn cancel_withdrawal(&self, withdrawal_id: &str) -> Result<String, APIError> {
        let endpoint = format!("/api/v1/withdrawals/{}", withdrawal_id);
        let url = format!("{}{}", &self.prefix, endpoint);
        let headers = self
            .sign_headers(endpoint, None, None, Method::DELETE)
            .unwrap();
        let resp = self.delete(url, Some(headers)).await?;
        let api_data = resp.text().await?;
        Ok(api_data)
    }
}
