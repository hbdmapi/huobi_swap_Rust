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