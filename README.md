# kucoin_rs

kucoin_rs is an open source library SDK/API wrapper for the 
[Kucoin Cryptocurrency Exchange](https://www.kucoin.com/).

Trading cryptocurrencies is high risk and there are no guarentees towards the stability or effectiveness
of this project. Comments, contributions, stars and donations are, however, all welcome.

## Description

kucoin_rs supports all currently available Kucoin REST and Websocket endpoints. It is designed to be 
async and relies primarily on the tokio async runtime, reqwest for the HTTP layer and tokio_tungstenite 
for the Websocket layer.

For the official API documentation visit [Kucoin Docs](https://docs.kucoin.com/).

For the library specific documentation please visit [kucoin_rs](https://docs.rs/kucoin_rs)

## Getting Started

The library can be used either directly through cloning the git repository and directly linking to your project or by utilizing cargo 
and installing the desired version. Once the library is accessible, bring the extern crate into your project. 

If you need information on particular endpoints please see the library specific documentation. If you clone the git you can run
`cargo doc --open --no-deps` to view them locally. Alternatively, you can run `cargo doc --open` on your own projects that have added
`kucoin_rs` as a depedency. Lastly you can visit [kucoin_rs](https://docs.rs/kucoin_rs).

## Authorization

Authorization is required for many of the endpoints. The [`Kucoin Client`] handles all
header construction but does require that the client is initialized with credentials to do so. To include credentials do the following:

```rust
use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};

let credentials = Credentials::new(
        "xxxxxxxxxxxxxXXXXXXxxx".to_string(),           // API KEY
        "XXxxxxx-xxxxxx-xXxxxx-xxxx".to_string(),       // SECRET KEY
        "xxxxxx".to_string()                            // PASSPHRASE
    );

let api = Kucoin::new(KucoinEnv::Live, Some(credentials));
```
A non-authorized client can be used for accessing Public Endpoints by inputting a None: `Kucoin::new(KucoinEnv::Live, None);`

## Contribution

Contributions are more than welcome for fixing bugs, writing further documentation, writing further tests, 
adding features or helping to improve performance. I'll do my best to review and implement pull requests.

## Donations

Donations are always appreciated and help support development of more open source trading tools.

BTC: 3KvTuAnv7o2VAf4LGgg1MiDURd2DgjFGaa

ETH: 0x7713a223e0e86355ac02b1e0de77695e822071cf

NEO: AWngpjmoXPHiJH6rtf81brPiyPomYAqe8j

Contact me for any other specific cryptocurrencies you'd prefer to use.

## License

This project is open source and uses the MIT license. Feel free to utilize it in whatever way you see fit.

