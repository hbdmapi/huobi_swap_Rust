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
    let access_key = "YOUR_ACCESS_KEY";
    let secret_key = "YOUR_SECRET_KEY";
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