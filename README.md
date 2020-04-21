# hbdm_swap

Rust Library for the [HBDM SWAP API](https://huobiapi.github.io/docs/coin_margined_swap/v1/cn/)

[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

## Risk Warning

Use at your own risk. We will not be responsible for your investment losses.

Cryptocurrency investment is subject to high market risk.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
hbdm_swap = { git = "https://github.com/hbdmapi/hbdm_swap_Rust.git" }
```

## Rust >= 1.37

```rust
rustup install stable
```

### MARKET DATA

```rust
extern crate hbdm_swap;
use hbdm_swap::*;
// extern crate simple_logger;

fn main() {
    // simple_logger::init().unwrap();
    let client = Client::new("", "");

    // get contract info
    match client.get_contract_info() {
        Ok(pairs) => println!(
            "contract info: {:?}",
            pairs,
        ),
        Err(why) => println!("error: {}", why),
    }

    //get orderbook
    match client.get_orderbook("BTC-USD", "step0") {
        Ok(orderbook) => println!(
            "orderbook info : {:?}", 
            orderbook,
        ),
        Err(why) => println!("error:{}", why),
    }

    //get klines
    match client.get_klines("BTC-USD", "1min", 10, None, None) {
        Ok(klines) => println!(
            "klines: {:?} ", 
            klines,
        ),
        Err(why) => println!("error: {}", why),
    }

    // get index data
    match client.get_swap_index("BTC-USD".to_string()) {
        Ok(index) => println!(
            "index: {:?} ", 
            index,
        ),
        Err(why) => println!("error: {}", why),
    }

    // get price limit
    match client.get_price_limit("BTC-USD".to_string()) {
        Ok(index) => println!(
            "price limit: {:?}",
            index,
        ),
        Err(why) => println!("error: {}", why),
    }

    // get open interest
    match client.get_open_interest("BTC-USD".to_string()) {
        Ok(index) => println!(
            "open interest: {:?}", 
            index,
        ),
        Err(why) => println!("error: {}", why),
    }

    // get merged market data
    match client.get_market_merged("BTC-USD") {
        Ok(market_merged) => println!(
            "merged data: {:?}", 
            market_merged,
        ),

        Err(why) => println!("error: {}", why),
    }

    // get market data
    match client.get_market_trade("BTC-USD") {
        Ok(market_trade) => println!(
            "market trade: {:?}",
            market_trade
        ),

        Err(why) => println!("error: {}", why),
    }

    // get market history data
    match client.get_market_history_trade("BTC-USD", 2) {
        Ok(history_trade) => println!(
            "history_trade: {:?}", 
            history_trade
        ),

        Err(why) => println!("error: {}", why),
    }

    // get risk info
    match client.get_risk_info("BTC-USD".to_string()) {
        Ok(risk_info) => println!(
            "risk_info: {:?}", risk_info
        ),

        Err(why) => println!("error: {}", why),
    }

    // get insurance fund
    match client.get_insurance_fund("BTC-USD") {
        Ok(insurance_fund) => println!(
            "insurance_fund: {:?}", insurance_fund
        ),
        Err(why) => println!("error: {}", why),
    }

    // get adjust factor
    match client.get_adjust_factor("BTC-USD".to_string()) {
        Ok(adjust_factor) => println!(
            "adjust_factor: {:?}", adjust_factor
        ),

        Err(why) => println!("error: {}", why),
    }

    // get history open interest
    match client.get_his_open_interest("BTC-USD", "1day", None, 1) {
        Ok(open_interest) => println!(
            "open_interest: {:?}", open_interest
        ),

        Err(why) => println!("error: {}", why),
    }

    // get elite account ratio
    match client.get_elite_account_ratio("BTC-USD", "1day") {
        Ok(elite_account_ratio) => println!(
            "elite_account_ratio: {:?}", elite_account_ratio
        ),

        Err(why) => println!("error: {}", why),
    }

    // get elite position ratio
    match client.get_elite_position_ratio("BTC-USD", "1day") {
        Ok(elite_position_ratio) => println!(
            "elite_position_ratio: {:?}", elite_position_ratio
        ),

        Err(why) => println!("error: {}", why),
    }

    //  get api state
    match client.get_api_state("BTC-USD".to_string()) {
        Ok(api_state) => println!(
            "api_state: {:?}", api_state
        ),

        Err(why) => println!("error: {}", why),
    }

    //  get funding rate
    match client.get_funding_rate("BTC-USD".to_string()) {
        Ok(funding_rate) => println!(
            "funding_rate: {:?}", funding_rate
        ),

        Err(why) => println!("error: {}", why),
    }

    // get history funding rate
    match client.get_his_funding_rate("BTC-USD", None, None) {
        Ok(his_funding_rate) => println!(
            "his_funding_rate: {:?}", his_funding_rate
        ),

        Err(why) => println!("error: {}", why),
    }

    // get liquidation orders
    match client.get_liquidation_orders("BTC-USD", 0, 7, None, None) {
        Ok(liquidation_order) => println!(
            "liquidation_order: {:?}", liquidation_order
        ),

        Err(why) => println!("error: {}", why),
    }

}

```

### ACCOUNT DATA

```rust
extern crate log;
extern crate rand;
// extern crate simple_logger;
extern crate hbdm_swap;
use hbdm_swap::*;
use rand::Rng;

fn main() {
    // simple_logger::init().unwrap();
    let client = Client::new("YOUR_API_ACCESSKEY", "YOUR_API_SECRETKEY");

    let masteraccount_uid = "your master account uid".to_string(); 

    // your subaccount uid
    let subaccount_uid = 100000;

    let mut rng = rand::thread_rng();

    let client_order_id: u32 = rng.gen();


     //transfer between spot and swap
     match client.spot_account_transfer("spot", "swap", "btc", 0.0009) {
        Ok(transfer_result) => println!(
            "transfer_result:\n {:?}", 
            transfer_result
        ),

        Err(why)=> println!("error: {}", why),
    }

    // transfer between master and subaccount
    match client.swap_master_sub_transfer(masteraccount_uid, "btc-usd".to_string(), 0.0001, "master_to_sub".to_string()) {
        Ok(transfer) =>
            println!(
                "transfer: \n {:?}",
                transfer
            ),
        Err(why) => println!("error: {}", why),
    }

    // place an order
    match client.place_order("btc-usd", client_order_id, 9999.1, 1, "sell", "open", 20, "limit" ) {
        Ok(order) => println!(
            "order: \n {:?}",
            order
        ),
        Err(why) => println!("{}", why),
    }

    //place orders
    let orders = BatchOrderRequest {
            orders_data: vec![
                    OrderRequest{
                        contract_code: "btc-usd".to_string(),
                        client_order_id: None,
                        price: Some(9999.1),
                        volume: 1,
                        direction: "sell".to_string(),
                        offset: "open".to_string(),
                        lever_rate: 20,
                        order_price_type: "limit".to_string(),
                    }
            ]

    };

    match client.place_orders(orders) {
        Ok(orders) => println!(
            "orders: \n {:?}",
            orders
        ),
        Err(why) => println!("{}", why),
    }

    // get account info
    match client.get_account("btc-usd".to_string()) {
        Ok(accounts) => println!(
            "accounts:\n{:?}",
            accounts
        ),
        Err(why) => println!("{}", why),
    }

    //get position info
    match client.get_position_info("BTC-USD".to_string()) {
        Ok(position_info) => println!(
            "position_info:\n {:?}",
            position_info
        ),
        Err(why) => println!("{}", why),
    }

    // get sub account list
    match client.get_sub_account_list("btc-usd".to_string()) {
        Ok(subaccountinfo) => 
            println!(
                "subaccountinfo: \n {:?}",
                subaccountinfo
            ),
        Err(why) => println!("{}", why),
    }

    // cancel orders
    match client.cancel_orders("".to_string(), client_order_id.to_string(), "btc-usd".to_string()) {
        Ok(orders) => println!(
            "orders: \n {:?}",
            orders
        ),
        Err(why) => println!("{}", why),
    }

    // cancel all orders
    match client.cancel_all("BTC-USD".to_string()) {
        Ok(orders) => println!(
            "orders: \n {:?}",
            orders
        ),
        Err(why) => println!("{}", why),
    }

    // get subaccountinfo
    match client.get_sub_account_info("btc-usd".to_string(), subaccount_uid) {
        Ok(subaccountinfo) => 
            println!(
                "subaccountinfo: \n {:?}",
                subaccountinfo
            ),
        Err(why) => println!("{}", why),
    }

    // get sub account position info
    match client.get_sub_position_info("btc-usd".to_string(), subaccount_uid) {
        Ok(subpositioninfo) => 
            println!(
                "subpositioninfo: \n {:?}",
                subpositioninfo
            ),
        Err(why) => println!("{}", why),
    }

    // get financial record
    match client.get_financial_record("btc-usd".to_string(), None, None, None, None) {
        Ok(financialrecord) => 
            println!(
                "financial_record: \n {:?}",
                financialrecord
            ),
        Err(why) => println!("{}", why),
    }

    // get order limit
    match client.get_order_limit("btc-usd".to_string(), "limit".to_string() ) {
        Ok(orderlimit) => 
            println!(
                "orderlimit: \n {:?}",
                orderlimit
            ),
        Err(why) => println!("{}", why),
    }

    // get transfer limit
    match client.get_transfer_limit("btc-usd".to_string()) {
        Ok(transfer_limit) => 
            println!(
                "transfer_limit: \n {:?}",
                transfer_limit
            ),
        Err(why) => println!("{}", why),
    }
    
    // get position limit
    match client.get_position_limit("btc-usd".to_string()) {
        Ok(position_limit) => 
            println!(
                "position_limit: \n {:?}",
                position_limit
            ),
        Err(why) => println!("{}", why),
    }

   
    // get api trading status
    match client.get_api_trading_status() { 
        Ok(api_trading_status) => println!(
            "api_trading_status: \n {:?}",
            api_trading_status
        ),
        Err(why) => println!("error: {}", why),
    }

    // get transfer records of master and subaccounts.
    match client.get_master_sub_transfer_record("btc-usd".to_string(), None, 1, None, None){
        Ok(transfer_records) => println!(
            "transfer records: \n {:?}",
            transfer_records
        ),
        Err(why) => println!("error: {}", why),
    }
    
    //get swap order info
    match client.get_order_info(None, client_order_id.to_string(), "btc-usd".to_string()) {
        Ok(order_info) => println!(
            "order info:\n {:?}",
            order_info
        ),
        Err(why) => println!("error: {}", why),
    } 

    //get swap order detail
    match client.get_order_detail("BTC-USD".to_string(), 699204501151698944, None,  1, None, None) {
        Ok(order_info) => println!(
            "order info:\n {:?}",
            order_info
        ),
        Err(why) => println!("error: {}", why),
    } 

    //get swap open orders
    match client.get_open_orders("btc-usd".to_string(), None, None) {
        Ok(open_orders) => println!(
            "open orders:\n {:?}",
            open_orders
        ),
        Err(why) => println!("error: {}", why),
    } 

    // get swap history orders
    match client.get_his_orders("btc-usd".to_string(), 0, 1, "0".to_string(), 1, None, None) {
        Ok(his_orders) => println!(
            "history orders:\n {:?}",
            his_orders
        ),
        Err(why) => println!("error: {}", why),
    } 

    // get swap match results
    match client.get_match_results("BTC-USD".to_string(), 0, 1, None, None) {
        Ok(match_results) => println!(
            "match_results:\n {:?}",
            match_results
        ),
        Err(why) => println!("error: {}", why),
    }
    
    //lightning close position
    match client.lightning_close("btc-usd".to_string(), 1, "sell".to_string(), None, None) {
        Ok(lightning_close) => println!(
            "lightning_close:\n {:?}",
            lightning_close
        ),
        Err(why) => println!("error: {}", why),
    }

   

}

```

### Websocket Subscription of Market Data

```rust
extern crate hbdm_swap;
use hbdm_swap::*;
use std::sync::atomic::{AtomicBool};
// extern crate simple_logger;

fn main() {
    market_websocket();
}

fn market_websocket() {
    // simple_logger::init().unwrap();
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let marketws: String = "/swap-ws".to_string();
    let mut web_socket: websocket::WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::OrderBook(order_book) => {
                println!(
                    "orderbook:{:?}", order_book
                );
            },
            WebsocketEvent::TradeDetail(trade_detail) => {
                println!(
                    "trade_detail:{:?}", trade_detail
                );
            },
            WebsocketEvent::Klines(klines) => {
                println!(
                    "Klines:{:?}", klines
                );
            },
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&marketws, vec!["BTC-USD"], vec!["orderbook", "kline.1min", "trade", "partial_orderbook"]).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => { 
               println!("Error: {}", err);
            }
        }
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}
```

### Websocket Subscription of User Stream

```rust
extern crate hbdm_swap;
use hbdm_swap::*;
use std::sync::atomic::{AtomicBool};
// extern crate simple_logger;

fn main() {
    account_websocket();
}

fn account_websocket() {
    // simple_logger::init().unwrap();
    let keep_running = AtomicBool::new(true);
    let access_key = "YOUR_API_ACCESSKEY";
    let secret_key = "YOUR_API_SECRETKEY";
    let accountws: String = "/swap-notification".to_string();
    let mut web_socket: WebSockets = WebSockets::new( | event: WebsocketEvent | {
        match event {
            WebsocketEvent::AccountUpdate(account_info) => {
                println!(
                    "account: {:?}", account_info
                );
            },
            WebsocketEvent::OrderUpdate(order_info) => {
                println!(
                    "order update: {:?}", order_info
                );
            },
            WebsocketEvent::PositionUpdate(position_info) => {
                println!(
                   "position update: {:?}", position_info
                );
            },
            WebsocketEvent::LiquidationUpdate(liquidation_info) => {
                println!(
                   "Liquidation update: {:?}", liquidation_info
                );
            },
            WebsocketEvent::FundingRateUpdate(funding_rate) => {
                println!(
                   "FundingRate update: {:?}", funding_rate
                );
            },
            _ => (),
        };

        Ok(())
    });

    web_socket.connect_auth(&accountws, vec!["BTC-USD"], vec!["account", "order", "position", "liquidation_order", "funding_rate"], access_key, secret_key).unwrap();
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
                println!("Error: {}", err);
            }
        }
    }

    web_socket.disconnect().unwrap();
    println!("disconnected");
}
```
