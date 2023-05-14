use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

/// Formats a query from a provided referenced hash map. Note, ordering is not assured.
pub fn format_query<S: ::std::hash::BuildHasher>(params: &HashMap<String, String, S>) -> String {
    let mut query = String::new();
    for (key, val) in params.iter() {
        let segment = format!("{}={}", key, val);
        if query.is_empty() {
            query = format!("?{}", segment);
        } else {
            query = format!("{}&{}", query, segment);
        }
    }
    query
}

#[cfg(test)]
mod test {
    use crate::utils::format_query;
    use std::collections::HashMap;
    #[test]
    fn format_query_test() {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("quantity".to_string(), 0.51.to_string());
        params.insert("symbol".to_string(), "BTC-USDT".to_string());
        params.insert("price".to_string(), 124.12.to_string());

        let query = format_query(&params);
        assert_eq!(query.contains("symbol=BTC-USDT"), true);
        assert_eq!(query.contains("price=124.12"), true);
        assert_eq!(query.contains("quantity=0.51"), true);
    }
}
