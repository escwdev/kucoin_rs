# kucoin_api
[![](https://img.shields.io/crates/v/kucoin-api)](https://crates.io/crates/kucoin-api)
[![](https://img.shields.io/docsrs/kucoin_api)](https://docs.rs/kucoin_api)
[![](https://img.shields.io/github/license/kanekoshoyu/kucoin_api)](https://github.com/kanekoshoyu/kucoin_api/blob/master/LICENSE)  
kucoin_api is an API wrapper for the [Kucoin Cryptocurrency Exchange](https://www.kucoin.com/). This is a derived project from Eric Abrahams's [kucoin_rs](https://github.com/escwdev/kucoin_rs).

Trading cryptocurrencies is high risk and there are no guarentees towards the stability or effectiveness
of this project. Comments, contributions, stars and donations are, however, all welcome.

## Description

kucoin_api supports all currently available Kucoin REST and Websocket endpoints. It is designed to be 
async and relies primarily on the tokio async runtime, reqwest for the HTTP layer and tokio_tungstenite 
for the Websocket layer.

For the official API documentation visit [Kucoin Docs](https://docs.kucoin.com/).

For the library specific documentation please visit [kucoin_api](https://docs.rs/kucoin_api)

## Getting Started

The library can be used either directly through cloning the git repository and directly linking to your project or by utilizing cargo 
and installing the desired version. Once the library is accessible, bring the extern crate into your project. 

If you need information on particular endpoints please see the library specific documentation. If you clone the git you can run
`cargo doc --open --no-deps` to view them locally. Alternatively, you can run `cargo doc --open` on your own projects that have added
`kucoin_api` as a depedency. Lastly you can visit [kucoin_api](https://docs.rs/kucoin_api).

## Authorization

Authorization is required for many of the endpoints. The [`Kucoin Client`] handles all
header construction but does require that the client is initialized with credentials to do so. To include credentials do the following:

```rust
use kucoin_api::client::{Kucoin, Credentials, KucoinEnv};

let credentials = Credentials::new(
        "xxxxxxxxxxxxxXXXXXXxxx",           // API KEY
        "XXxxxxx-xxxxxx-xXxxxx-xxxx",       // SECRET KEY
        "xxxxxx"                            // PASSPHRASE
    );

let api = Kucoin::new(KucoinEnv::Live, Some(credentials));
```
A non-authorized client can be used for accessing Public Endpoints by inputting a None: `Kucoin::new(KucoinEnv::Live, None);`

## Contribution

Contributions are more than welcome for fixing bugs, writing further documentation, writing further tests, 
adding features or helping to improve performance. I'll do my best to review and implement pull requests.

## License

This project is open source and uses the MIT license. Feel free to utilize it in whatever way you see fit.

