extern crate hbdm_swap;
use hbdm_swap::*;
// extern crate simple_logger;

fn main() {
    // simple_logger::init().unwrap();
    let client = Client::new("YOUR_API_KEY", "YOUR_SECRET_KEY");

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
