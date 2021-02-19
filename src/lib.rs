//! # kucoin_rs
//! kucoin_rs is an open source library SDK/API wrapper for the
//! [Kucoin Cryptocurrency Exchange](https://www.kucoin.com/).
//!
//! Trading cryptocurrencies is high risk and there are no guarentees towards the stability or effectiveness
//! of this project. Comments, contributions, stars and donations are, however, all welcome.
//!
//! ## Description
//!
//! kucoin_rs supports all currently available Kucoin REST and Websocket endpoints. It is designed to be
//! async and relies primarily on the tokio async runtime, reqwest for the HTTP layer and tokio_tungstenite
//! for the Websocket layer.
//!
//! For the official API documentation visit [Kucoin Docs](https://docs.kucoin.com/).
//!
//! Please be aware that due to the nature of a number of endpoints, the response structs and input parameters of
//! several requests may contain Option\<T\> and will require appropriate pattern matching. These generally coincide
//! with optional input params which can be seen by visiting the official Kucoin API documentation noted above.
//!
//! See the Kucoin Client for all endpoint fn calls and required/optional input types, and endpoint models for specifics: <br />
//! * [`Kucoin Client`](./kucoin/client/struct.Kucoin.html)
//! * [`API General Response Models`](./kucoin/model/index.html)                  
//! * [`Market Response Models`](./kucoin/model/market/index.html)
//! * [`Margin Response Models`](./kucoin/model/margin/index.html)
//! * [`Trade Response Models`](./kucoin/model/trade/index.html)        
//! * [`User Response Models`](./kucoin/model/user/index.html)          
//! * [`Websocket Response Models`](./kucoin/model/websocket/index.html)
//!
//! These project docs also provide details regarding necessary input parameters and response structs,
//! helping to identify cases where Option\<T\> matching is and is not necessary.
//!
//! ## Getting Started
//!
//! The library can be used either directly through the git repository or by utilizing cargo and installing the desired version. Once
//! the library is accessible, simply bring the extern crate into your project.
//!
//! If you want information on particular endpoints, please review the library documentation.
//!
//! ### Authorization
//!
//! Authorization is required for many of the endpoints. The [`Kucoin Client`](./kucoin/client/struct.Kucoin.html) handles all
//! header construction but does require that the client is initialized with credentials to do so. To include credentials do the following:
//!
//! ```
//! use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
//!
//! let credentials = Credentials::new(
//!         "xxxxxxxxxxxxxXXXXXXxxx",           // API KEY
//!         "XXxxxxx-xxxxxx-xXxxxx-xxxx",       // SECRET KEY
//!         "xxxxxx"                            // PASSPHRASE
//!     );
//!
//! let api = Kucoin::new(KucoinEnv::Live, Some(credentials));
//! ```
//! A non-authorized client can be used for accessing Public Endpoints by inputting a None: `Kucoin::new(KucoinEnv::Live, None);`
//!
//! ## Examples
//!
//! Below are some basic examples.
//!
//! Private endpoints require an authorized client. Check above for further details on initializing kucoin_rs
//! with appropriate credentials to access private endpoints
//!
//! ### REST Usage
//!
//! REST calls, like Websocket connections, require first setting up the client. Once the client is setup, calls can be made in whatever
//! ways suit end-users' needs.
//!
//! Please note that endpoints have varying amounts of input parameter requirements and options. Required parameters are always direct inputs
//! but types may vary. Optional requirements are wrapped in Option\<T\>, so be aware that a large number of calls require None or Some(T).
//! inputs. The endpoints with signficant number of options take advantage of builder methods on optional structs.
//! This documention provides details of where this is necessary. To check for specific endpoints, see:
//! [`Kucoin Client`](./kucoin/client/struct.Kucoin.html). Optional structs with builders will be identified in the fn signatures.
//!
//! A simple example is:
//!
//! ```ignore
//! extern crate kucoin_rs;
//!
//! use kucoin_rs::tokio;
//! use kucoin_rs::failure;
//! use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), failure::Error>  {
//!     let api = Kucoin::new(KucoinEnv::Sandbox, None)?;
//!     let result = api.get_ticker("BTC-USDT").await?;
//!     match result.data {
//!         Some(d) => println!("{:#?}", d),
//!         None => println!("Code: {}, msg: {:#?}", result.code, result.msg),
//!     }   
//!     Ok(())
//! }
//! ```
//!
//! An example with custom error handling is:
//!
//! ```ignore
//! extern crate kucoin_rs;
//!
//! use kucoin_rs::tokio;
//! use kucoin_rs::failure;
//! use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
//! use kucoin_rs::kucoin::error::APIError;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), failure::Error>  {
//!    let result = api.get_server_time().await;
//!    match result {
//!        Err(e) => {
//!            match e {
//!                APIError::HTTP(e) => eprintln!("Reqwest Error: {}", e),
//!                _ => eprintln!("Non HTTP Error: {}", e),
//!            }
//!        },
//!        Ok(s) => {
//!            match s.data {
//!                Some(d) => println!("{:#?}", d),
//!                None => println!("Code: {}, msg: {:#?}", s.code, s.msg),
//!            }
//!        }
//!    }         
//!    Ok(())
//! }
//! ```
//!
//!
//! ### Websocket Usage
//!
//! Websockets require several steps to initalize. A single websocket can accept up to 10 subscriptions,
//! as per Kucoin limitations. Due to this, the instantiation of the socket takes a Vec\<[WSTopic](./kucoin/model/websocket/enum.WSTopic.html)\>.
//! The reason is because multiple subscriptions can be initialized from one call. Below is a simplified single subscription with a line-by-line
//! short explanation including some basic options for specified error handling.
//!
//! ```ignore
//! extern crate kucoin_rs;
//!
//! use kucoin_rs::tokio;
//! use kucoin_rs::failure;
//! use kucoin_rs::tokio::stream::StreamExt;
//!
//! use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
//! use kucoin_rs::kucoin::model::websocket::{Subscribe, KucoinWebsocketMsg, WSType, WSTopic, WSResp};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), failure::Error>  {
//!     // If credentials are needed, generate a new Credentials struct w/ the necessary keys
//!     let credentials = Credentials::new(
//!         "xxxxxxxxxxxxxXXXXXXxxx",
//!         "XXxxxxx-xxxxxx-xXxxxx-xxxx",
//!         "xxxxxx"
//!     );
//!
//!     // Initialize the Kucoin API struct
//!     let api = Kucoin::new(KucoinEnv::Live, Some(credentials))?;
//!     
//!     // Generate the dynamic Public or Private websocket url and endpoint from Kucoin
//!     // which includes a token required for connecting
//!     let url = api.get_socket_endpoint(WSType::Public).await?;
//!     
//!     // Initialize the websocket
//!     let mut ws = api.websocket();
//!
//!     // Generate a Vec<WSTopic> of desired subs. Note they need to be public or private
//!     // depending on the url
//!     let subs = vec![WSTopic::Ticker(vec!["BTC-USDT".to_string()])];
//!     
//!     // Initalize your subscription and use await to unwrap the future   
//!     ws.subscribe(url, subs).await?;
//!     
//!     // Handle incoming responses matching messages. Note, the message matching is
//!     // not required for a single subscription but may be desired
//!     // for more complex event handling for multi-subscribed sockets add the additional
//!     // KucoinWebSocketMsg matches.
//!     while let Some(msg) = ws.try_next().await? {
//!         match msg {
//!             KucoinWebsocketMsg::TickerMsg(msg) => println!("{:#?}", msg),
//!             KucoinWebsocketMsg::PongMsg(msg) => println!("{:#?}", msg),     // Optional
//!             KucoinWebsocketMsg::WelcomeMsg(msg) => println!("{:#?}", msg),  // Optional
//!             _ => (),
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//! [`KucoinWebsocketMsg`](./kucoin/model/websocket/enum.KucoinWebsocketMsg.html) has all the message response types
//! available and can be referenced to identify desired endpoints.
//!
//! [`WSTopic`](./kucoin/model/websocket/enum.WSTopic.html) has all the available websocket topics/endpoints that are
//! available for subscription.
//!
//! Note that Level3 data has been separated by message type despite it requiring only a single subscription.
//! All other subscriptions coincide 1:1 with their response type and KucoinWebsocketMsg,
//! excluding their Ping, Pong and Welcome messages. Ping, Pong and Welcome can be tracked through their own match arm.
//! The reasoning for this exception is that for the majority of use cases, each Level3 message has to be handled
//! in its own way to properly construct an orderbook. By separating the messages by type from the incoming
//! stream at the library level, it helps to reduce duplication for the end user.
//!
//! ## Error Handling
//!
//! kucoin_rs uses the [`failure crate`](https://crates.io/crates/failure) to propagate errors. Kucoin REST errors are
//! passed as part of the response structs, however by default, reqwest errors panic. For websocket endpoints, similarly,
//! by default most protocol and connection errors will panic. Use of `?` will result in panics as well. End users can however  
//! use the custom [`APIError`](./kucoin/error/enum.APIError.html) enum to match error responses which provide non panic
//! alternatives allowing for specified error handling. Users can also implement their own more comprehensive solutions.
//!
//! ## Contribution
//!
//! Contributions are more than welcome for fixing bugs, writing further documentation, writing further tests,
//! adding features or helping to improve performance. I'll do my best to review and implement pull requests.
//!
//! ## Donations
//!
//! Donations are always appreciated and help support development of more open source trading tools.
//!
//! BTC: 3KvTuAnv7o2VAf4LGgg1MiDURd2DgjFGaa
//!
//! ETH: 0x7713a223e0e86355ac02b1e0de77695e822071cf
//!
//! NEO: AWngpjmoXPHiJH6rtf81brPiyPomYAqe8j
//!
//! Contact me for any other specific cryptocurrencies you'd prefer to use.
//!
//! ## License
//!
//! This project is open source and uses the MIT license. Feel free to utilize it in whatever way you see fit.

pub extern crate futures;
pub extern crate pin_project;
pub extern crate reqwest;
pub extern crate tokio;
pub extern crate tokio_native_tls;
pub extern crate tokio_tungstenite;
pub extern crate tungstenite;
pub extern crate url;

pub extern crate serde;
pub extern crate serde_json;

#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate failure;

/// Kucoin API Module
pub mod kucoin;
