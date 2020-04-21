extern crate log;
extern crate rand;
// extern crate simple_logger;
extern crate hbdm_swap;
use hbdm_swap::*;
use rand::Rng;

fn main() {
    // simple_logger::init().unwrap();
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

    let masteraccount_uid = "51031836".to_string(); 
    let subaccount_uid = 112916395;

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
    match client.place_order("btc-usd", client_order_id, 9999.1, 1, "sell", "open", 125, "limit" ) {
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
