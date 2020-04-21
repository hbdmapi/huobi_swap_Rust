use crate::{error::*, models::*, client::*};
use std::io::prelude::*;
use std::collections::BTreeMap;
use url::Url;
use serde_json::{json};

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use std::sync::Mutex;

static WEBSOCKET_URL: &'static str = "wss://api.hbdm.com";
static WS_HOST: &'static str = "api.hbdm.com";

static KLINE: &'static str = "kline";
static TRADE: &'static str = "trade.detail";
static DEPTH_ORDERBOOK : &'static str = "depth";
static PARTIAL_ORDERBOOK : &'static str = "high_freq";

lazy_static! {
    static ref SYMBOLS: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref CHANNELS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub enum WebsocketEvent {
    Klines(Klines),
    TradeDetail(Trade),
    OrderBook(OrderBook),
    AccountUpdate(AccountInfo),
    OrderUpdate(OrderSubs),
    PositionUpdate(PositionSubs),
    LiquidationUpdate(LiquidationSubs),
    FundingRateUpdate(FundingRateSubs),
}

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> APIResult<()> + 'a>,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> APIResult<()> + 'a
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, endpoint: &str, symbols: Vec<&str>, channels: Vec<&str>) -> APIResult<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;
        ::log::info!("url:{}", url);
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                for channel in &channels {
                    if channel.contains("orderbook") && !channel.contains("partial_orderbook") {
                        for symbol in &symbols { 
                            let message = json!({
                                "sub": format!("market.{}.depth.step6", symbol),       
                                "id": "rustclient"
                            }); 
                            if let Some(ref mut socket) = self.socket {
                                socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                ::log::info!("Write message {}", message.to_string());
                            };
                        }
                    }
                    if channel.contains("partial_orderbook") {
                        for symbol in &symbols { 
                            let message = json!({
                                "sub": format!("market.{}.depth.size_150.high_freq", symbol),       
                                "data_type": "incremental",
                                "id": "rustclient"
                            }); 
                            if let Some(ref mut socket) = self.socket {
                                socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                ::log::info!("Write message {}", message.to_string());
                            };
                        }
                    }

                    if channel.contains("kline") {
                        for symbol in &symbols { 
                            let message = json!({
                                "sub": format!("market.{}.{}", symbol, channel),       
                                "id": "rustclient"
                            }); 
                            if let Some(ref mut socket) = self.socket {
                                socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                ::log::info!("Write message {}", message.to_string());
                            };
                        }
                    }


                    else if channel.contains("trade") {
                        for symbol in &symbols { 
                            let message = json!({
                                "sub": format!("market.{}.trade.detail", symbol),       
                                "id": "rustclient"
                            }); 
                            if let Some(ref mut socket) = self.socket {
                                socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                ::log::info!("Write message {}", message.to_string());
                            };
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                ::log::info!("Error during handshake {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn connect_auth(&mut self, endpoint: &str,symbols: Vec<&str>, channels: Vec<&str>, access_key: &str, secret_key: &str ) -> APIResult<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;
        ::log::info!("url:{}", url);

        for symbol in symbols {
            SYMBOLS.lock().unwrap().push(symbol.to_string());
        }

        for channel in channels {
            CHANNELS.lock().unwrap().push(channel.to_string());
        }
    
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                let mut params: BTreeMap<String, String> = BTreeMap::new();
                params.insert("AccessKeyId".to_string(), access_key.to_string());
                params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
                params.insert("SignatureVersion".to_string(), "2".to_string());
                let utctime = get_timestamp();
                params.insert("Timestamp".to_string(), utctime.clone()); 
                // println!("params: {:?}", params.clone());

                let build_params = build_query_string(params.clone());

                let format_str = format!("{}\n{}\n{}\n{}", "GET", WS_HOST, endpoint, build_params,); 

                // println!("format str:{}", format_str.clone());

                let signature = sign_hmac_sha256_base64(
                    &secret_key,
                    &format_str,
                )
                .to_string();

                ::log::info!("signature: {}",signature.clone());

                let message = json!({
                    "AccessKeyId": params.get(&"AccessKeyId".to_string()),
                    "SignatureMethod": params.get(&"SignatureMethod".to_string()),
                    "SignatureVersion": params.get(&"SignatureVersion".to_string()),
                    "Timestamp": params.get(&"Timestamp".to_string()),
                    "Signature": signature,
                    "op": "auth".to_string(),
                    "type": "api".to_string(),   
                }); 
                if let Some(ref mut socket) = self.socket {
                    socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                    ::log::info!("Write message {}", message.to_string());
                };
                Ok(())
            }
            Err(e) => {
                ::log::info!("Error during handshake {}", e);
                Err(Box::new(e))
            }
        }
    }


    pub fn disconnect(&mut self) -> APIResult<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            Ok(())
        } else {
            ::log::info!("Not able to close the connection");
            Ok(())
        }
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> APIResult<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;

                match message {
                    Message::Text(_) => {}
                    Message::Ping(bin) |
                    Message::Pong(bin) |
                    Message::Binary(bin) => {
                        let mut d = GzDecoder::new(&*bin);
                        let mut s = String::new();
                        d.read_to_string(&mut s).unwrap();
                        let msg: serde_json::Value = serde_json::from_str(&s)?;
                        ::log::debug!("####:{:?}", msg);
                        if msg.get("ping") != None {
                            ::log::info!("####:{:?}", msg);
                            if let Some(ref mut socket) = self.socket {
                                let message = json!({
                                    "pong": msg.get("ping"),       
                                }); 
                                socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                ::log::info!("Write message {}", message.to_string());
                            };
                        } 
                        if let Some(op) = msg.get("op") {
                            if op == "ping" {
                                if let Some(ref mut socket) = self.socket {
                                    let message = json!({
                                        "op": "pong",
                                        "ts": msg.get("ts"),
                                        });
                                    socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                    ::log::info!("Write message {}", message.to_string());
                                }
                            }

                            if op == "auth" {
                                if let Some(err_code) = msg.get("err-code") {
                                    if err_code == 0 {
                                        for channel in &*CHANNELS.lock().unwrap() {
                                            if channel.contains("account") {
                                                for symbol in &*SYMBOLS.lock().unwrap() {
                                                    let message = json!({
                                                        "op": "sub",
                                                        "cid": "hbdm-rust",
                                                        "topic": format!("accounts.{}", symbol),
                                                        });

                                                    if let Some(ref mut socket) = self.socket {
                                                        socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                                        ::log::info!("Write message {}", message.to_string());
                                                    }
                                                
                                                }
                                            }

                                            if channel.contains("order") {
                                                for symbol in &*SYMBOLS.lock().unwrap() { 
                                                    let message = json!({
                                                        "op": "sub",
                                                        "cid": "hbdm-rust",
                                                        "topic": format!("orders.{}", symbol),
                                                    });

                                                    if let Some(ref mut socket) = self.socket {
                                                        socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                                        ::log::info!("Write message {}", message.to_string());
                                                    }
                                                }
                                            }
                                            
                                            if channel.contains("position") {
                                                for symbol in &*SYMBOLS.lock().unwrap() {
                                                    let message = json!({
                                                        "op": "sub",
                                                        "cid": "hbdm-rust",
                                                        "topic": format!("positions.{}", symbol),
                                                    });
    
                                                    if let Some(ref mut socket) = self.socket {
                                                        socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                                        ::log::info!("Write message {}", message.to_string());
                                                    }
                                                }
                                            }

                                            if channel.contains("liquidation_order") {
                                                for symbol in &*SYMBOLS.lock().unwrap() { 
                                                    let message = json!({
                                                        "op": "sub",
                                                        "cid": "hbdm-rust",
                                                        "topic": format!("public.{}.liquidation_orders", symbol),
                                                    });

                                                    if let Some(ref mut socket) = self.socket {
                                                        socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                                        ::log::info!("Write message {}", message.to_string());
                                                    }
                                                }
                                            }

                                            if channel.contains("funding_rate") {
                                                for symbol in &*SYMBOLS.lock().unwrap() {
                                                    let message = json!({
                                                        "op": "sub",
                                                        "cid": "hbdm-rust",
                                                        "topic": format!("public.{}.funding_rate", symbol),
                                                    });

                                                    if let Some(ref mut socket) = self.socket {
                                                        socket.0.write_message(tungstenite::Message::Text(message.to_string()))?;
                                                        ::log::info!("Write message {}", message.to_string());
                                                    }
                                                }
                                            }


                                        }

                                 
                                    }
                                }
                            }

                            if op == "notify" {
                                if let Some(topic) = msg.get("topic") {
                                    if topic.to_string().contains("accounts") {
                                        let account_update: AccountInfo = serde_json::from_str(&s)?;
                                        (self.handler)(WebsocketEvent::AccountUpdate(account_update))?;

                                    }

                                    if topic.to_string().contains("orders") {
                                        let order_update: OrderSubs = serde_json::from_str(&s)?;
                                        (self.handler)(WebsocketEvent::OrderUpdate(order_update))?;
                                    }

                                    if topic.to_string().contains("positions") {
                                        let position_update: PositionSubs = serde_json::from_str(&s)?;
                                        (self.handler)(WebsocketEvent::PositionUpdate(position_update))?;
                                    }

                                    if topic.to_string().contains("liquidation_orders") {
                                        let liquidation_orders: LiquidationSubs = serde_json::from_str(&s)?;
                                        (self.handler)(WebsocketEvent::LiquidationUpdate(liquidation_orders))?;
                                    }

                                    if topic.to_string().contains("funding_rate") {
                                        let funding_rate: FundingRateSubs = serde_json::from_str(&s)?;
                                        (self.handler)(WebsocketEvent::FundingRateUpdate(funding_rate))?;
                                    }

                                }
                            }
                        }

                        if let Some(data) = msg.get("ch") {
                            if let Some(topic) = data.as_str() {
                                if topic.contains(DEPTH_ORDERBOOK) == true {
                                    let depth_orderbook: OrderBook = serde_json::from_str(&s)?;
                                    (self.handler)(WebsocketEvent::OrderBook(depth_orderbook))?;
                                } 

                                if topic.contains(PARTIAL_ORDERBOOK) == true {
                                    let depth_orderbook: OrderBook = serde_json::from_str(&s)?;
                                    (self.handler)(WebsocketEvent::OrderBook(depth_orderbook))?;
                                } 

                                if topic.contains(TRADE) == true {
                                    let trade_detail: Trade = serde_json::from_str(&s)?;
                                    (self.handler)(WebsocketEvent::TradeDetail(trade_detail))?;
                                } 

                                if topic.contains(KLINE) == true {
                                    let klines: Klines = serde_json::from_str(&s)?;
                                    (self.handler)(WebsocketEvent::Klines(klines))?;
                                } 
                            }
                        };


                    }
                    Message::Close(e) => {
                        ::log::info!("Disconnected {:?}", e);
                    }
                }
            }
        }
        Ok(())
    }
}
