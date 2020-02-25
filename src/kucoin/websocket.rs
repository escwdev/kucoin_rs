use std::collections::HashMap;

use reqwest::header;
use url::Url;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time;
use std::time::Duration;
use futures::{prelude::*, stream::SplitStream};
use streamunordered::{StreamUnordered, StreamYield};
use pin_project::*;

use failure;
use serde_json;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::client::Kucoin;
use super::error::APIError;
use super::utils::get_time;
use super::model::{APIDatum, Method};
use super::model::websocket::{
    InstanceServers, 
    Subscribe, 
    WSTopic, 
    WSType, 
    DefaultMsg,
    KucoinWebsocketMsg};


type WSStream = WebSocketStream<tokio_tungstenite::stream::Stream<TcpStream, tokio_tls::TlsStream<TcpStream>>>;
pub type StoredStream = SplitStream<WSStream>;

#[pin_project]
#[derive(Default)]
pub struct KucoinWebsocket {
    subscriptions: HashMap<WSTopic, usize>,
    tokens: HashMap<usize, WSTopic>,
    heartbeats: HashMap<usize, tokio::task::JoinHandle<()>>,
    #[pin]
    streams: StreamUnordered<StoredStream>,
}

impl Stream for KucoinWebsocket {
    type Item = Result<KucoinWebsocketMsg, APIError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.as_mut().project().streams.poll_next(cx) {
            Poll::Ready(Some((y, _))) => match y {
                StreamYield::Item(item) => {
                    // let heartbeat = self.heartbeats.get_mut(&token).unwrap();
                    Poll::Ready({
                        Some(item.map_err(APIError::Websocket)
                            .and_then(|m| parse_message(m))
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => panic!("No Stream Subscribed"),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl KucoinWebsocket {
    pub async fn subscribe(&mut self, url: String, ws_topic: Vec<WSTopic>) -> Result<(), APIError> {
        let endpoint = Url::parse(&url).unwrap();
        let (ws_stream, _) = connect_async(endpoint).await?;

        let (sink, read) = ws_stream.split();
        let sink_mutex = Mutex::new(sink);
        
        for topic in ws_topic.iter() {
            let sub = Subscribe::new(topic);
            println!("Websocket subscribing to {}", &sub.topic);
        
            sink_mutex.lock()
                .await
                .send(Message::Text(serde_json::to_string(&sub).unwrap()))
                .await?;
        }

        // Ping heartbeat
        let heartbeat = tokio::spawn(async move {
            loop {
                time::delay_for(Duration::from_secs(30)).await;
                let ping = DefaultMsg {
                    id: get_time().to_string(),
                    r#type: "ping".to_string(),
                };
                let resp = sink_mutex.lock()
                    .await
                    .send(Message::Text(serde_json::to_string(&ping).unwrap()))
                    .map_err(APIError::Websocket)
                    .await;
        
                if let Err(e) = resp { 
                    match e {
                        APIError::Websocket(e) => {
                            eprintln!("Error sending Ping: {}", e);
                            break
                        }
                        _ => eprintln!("None websocket error sending Ping: {}", e),
                    }
                };
            }
        });
        let token = self.streams.push(read);
        self.heartbeats.insert(token, heartbeat);
        self.subscriptions.insert(ws_topic[0].clone(), token);
        self.tokens.insert(token, ws_topic[0].clone());

        Ok(())
    }

    pub fn unsubscribe(&mut self, ws_topic: WSTopic) -> Option<StoredStream> {
        let streams = Pin::new(&mut self.streams);
        println!("Unsubscribing from: {:?}", ws_topic);
        self.subscriptions
            .get(&ws_topic)
            .and_then(|token| StreamUnordered::take(streams, *token))
    }
}

fn parse_message(msg: Message) -> Result<KucoinWebsocketMsg, APIError> {
    match msg {
        Message::Text(msg) => {
            if msg.contains("\"type\":\"welcome\"") || msg.contains("\"type\":\"ack\"") {
                Ok(KucoinWebsocketMsg::WelcomeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"ping\"") {
                Ok(KucoinWebsocketMsg::PingMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"pong\"") {
                Ok(KucoinWebsocketMsg::PongMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.ticker\"") {
                Ok(KucoinWebsocketMsg::TickerMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"topic\":\"/market/ticker:all\"") {
                Ok(KucoinWebsocketMsg::AllTickerMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.snapshot\"") {
                Ok(KucoinWebsocketMsg::SnapshotMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l2update\"") {
                Ok(KucoinWebsocketMsg::OrderBookMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/market/match:") {
                Ok(KucoinWebsocketMsg::MatchMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3received\"") {
                Ok(KucoinWebsocketMsg::Level3ReceivedMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3open\"") {
                Ok(KucoinWebsocketMsg::Level3OpenMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3done\"") {
                Ok(KucoinWebsocketMsg::Level3DoneMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3match\"") {
                Ok(KucoinWebsocketMsg::Level3MatchMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"subject\":\"trade.l3change\"") {
                Ok(KucoinWebsocketMsg::Level3ChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/indicator/index:") {
                Ok(KucoinWebsocketMsg::IndexPriceMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/indicator/markPrice:") {
                Ok(KucoinWebsocketMsg::MarketPriceMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/margin/fundingBook:") {
                Ok(KucoinWebsocketMsg::OrderBookChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("\"type\":\"stop\"") || msg.contains("\"type\":\"activate\"") {
                Ok(KucoinWebsocketMsg::StopOrderMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("/account/balance") {
                Ok(KucoinWebsocketMsg::BalancesMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("debt.ratio")  {
                Ok(KucoinWebsocketMsg::DebtRatioMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("position.status") {
                Ok(KucoinWebsocketMsg::PositionChangeMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.open") {
                Ok(KucoinWebsocketMsg::MarginTradeOpenMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.update") {
                Ok(KucoinWebsocketMsg::MarginTradeUpdateMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("order.done") {
                Ok(KucoinWebsocketMsg::MarginTradeDoneMsg(serde_json::from_str(&msg)?))
            } else if msg.contains("error") {
                Ok(KucoinWebsocketMsg::Error(msg))
            } else {
                serde::export::Err(APIError::Other("No KucoinWebSocketMsg type to parse".to_string()))
            }
        }
        Message::Binary(b) => Ok(KucoinWebsocketMsg::Binary(b)),
        Message::Pong(..) => Ok(KucoinWebsocketMsg::Pong),
        Message::Ping(..) => Ok(KucoinWebsocketMsg::Ping),
        Message::Close(..) => {
            serde::export::Err(APIError::Other("Socket closed error".to_string()))
        }
    }
}

pub async fn close_socket(heartbeat: &mut tokio::task::JoinHandle<()>) -> Result<(), failure::Error> {
    heartbeat.await?;
    println!("Heartbeat turned off...");
    Ok(())
}

impl Kucoin {
    pub fn websocket(&self) -> KucoinWebsocket {
        KucoinWebsocket::default()
    }

    pub async fn ws_bullet_private(&self) -> Result<APIDatum<InstanceServers>, APIError> {
        let endpoint = String::from("/api/v1/bullet-private");
        let url: String = format!("{}{}", &self.prefix, endpoint);
        let header: header::HeaderMap = self.sign_headers(endpoint, None, None, Method::POST).unwrap();
        let resp = self.post(url, Some(header), None).await?;
        let api_data: APIDatum<InstanceServers> = resp.json().await?;
        Ok(api_data)
    }

    pub async fn ws_bullet_public(&self) -> Result<APIDatum<InstanceServers>, APIError> {
        let endpoint = String::from("/api/v1/bullet-public");
        let url: String = format!("{}{}", &self.prefix, endpoint);
        let header: header::HeaderMap = self.sign_headers(endpoint, None, None, Method::POST).unwrap();
        let resp = self.post(url, Some(header), None).await?;
        let api_data: APIDatum<InstanceServers> = resp.json().await?;
        Ok(api_data)
    }

    pub async fn get_socket_endpoint(&self, ws_type: WSType) -> Result<String, APIError> {
        let mut endpoint: String = String::new();
        let mut token: String = String::new();
        let timestamp = get_time();
        match ws_type {
            WSType::Private => {
                let resp = &self.ws_bullet_private().await?;
                if let Some(r) = &resp.data {
                    token = r.token.to_owned();
                    endpoint = r.instance_servers[0].endpoint.to_owned();
                }
            },
            WSType::Public => {
                let resp = &self.ws_bullet_public().await?;
                if let Some(r) = &resp.data {
                    token = r.token.to_owned();
                    endpoint = r.instance_servers[0].endpoint.to_owned();
                }
            },
        }
        let url = format!("{}?token={}&[connectId={}]?acceptUserMessage=\"true\"", endpoint, token, timestamp);
        Ok(url)
    }
}

impl Subscribe {
    pub fn new(topic_type: &WSTopic) -> Self {
        let id = get_time().to_string();
        let mut private_channel = false;
        let topic = match topic_type {
            WSTopic::Ticker(ref symbols) => format!("/market/ticker:{}", symbols.join(",")),
            WSTopic::AllTicker => String::from("/market/ticker:all"),
            WSTopic::Snapshot(ref symbol) => format!("/market/snapshot:{}", symbol),
            WSTopic::IndexPrice(ref symbols) => format!("/indicator/index:{}", symbols.join(",")),
            WSTopic::MarketPrice(ref symbols) => format!("/indicator/markPrice:{}", symbols.join(",")),
            WSTopic::OrderBook(ref symbols) => format!("/market/level2:{}", symbols.join(",")),
            WSTopic::OrderBookChange(ref symbols) => format!("/margin/fundingBook:{}", symbols.join(",")),
            WSTopic::Match(ref symbols) => format!("/market/match:{}", symbols.join(",")),
            WSTopic::Level3Public(ref symbols) => format!("/market/level3:{}", symbols.join(",")),
            WSTopic::Level3Private(ref symbols) => {
                private_channel = true;
                format!("/market/level3:{}", symbols.join(",")) 
            },
            WSTopic::Balances => {
                private_channel = true;
                String::from("/account/balance")
            },
            WSTopic::StopOrder(ref symbols) => {
                private_channel = true;
                format!("/market/level3:{}", symbols.join(",")) 
            },
            WSTopic::DebtRatio => {
                private_channel = true;
                String::from("/margin/position")
            },
            WSTopic::PositionChange => {
                private_channel = true;
                String::from("/margin/position")
            },
            WSTopic::MarginTradeOrder(ref symbol) => {
                private_channel = true;
                format!("/margin/loan:{}", symbol)
            },  
        };

        Subscribe {
            id,
            r#type: String::from("subscribe"),
            topic,
            private_channel,
            response: true
        }

    }
}


