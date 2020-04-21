use super::*;
//use crate::error::*;
//use crate::models::*;
use serde_json::from_str;

impl Client {
    // Get contract information (contract metadata etc)
    pub fn get_contract_info(&self) -> APIResult<ContractInfo> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let data: String = self.get("/swap-api/v1/swap_contract_info", &parameters)?;

        let info: ContractInfo = from_str(data.as_str())?;

        Ok(info)
    }

    // Get swap index
    pub fn get_swap_index<S1>(&self, contract_code: S1) -> APIResult<IndexInfo> 
    where S1: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(code) = contract_code.into() { parameters.insert("contract_code".into(), code); }
        
        let data: String = self.get("/swap-api/v1/swap_index", &parameters)?;

        let info: IndexInfo = from_str(data.as_str())?;

        Ok(info)
    }

    // Get swap price limit
    pub fn get_price_limit<S1>(&self, contract_code: S1) -> APIResult<PriceLimit> 
        where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        let data: String = self.get("/swap-api/v1/swap_price_limit", &parameters)?;

        let info: PriceLimit = from_str(data.as_str())?;

        Ok(info)
    }

    // Get open interest
    pub fn get_open_interest<S1>(&self, contract_code: S1) -> APIResult<OpenInterest>
        where S1: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(code) = contract_code.into() { parameters.insert("contract_code".into(), format!("{}", code));}

        let data: String = self.get("/swap-api/v1/swap_open_interest", &parameters)?;

        let info: OpenInterest = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Orderbook
    pub fn get_orderbook<S1, S2>(&self, contract_code: S1, orderbook_type: S2) -> APIResult<OrderBook>
        where S1: Into<String>, S2: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        parameters.insert("type".into(), orderbook_type.into());
        
        let data: String = self.get("/swap-ex/market/depth", &parameters)?;

        let info: OrderBook = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Kline
    pub fn get_klines<S1, S2, S3, S4, S5>(&self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5) -> APIResult<Klines>
    where S1: Into<String>, S2: Into<String>, S3: Into<Option<u32>>, S4: Into<Option<u64>>, S5: Into<Option<u64>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), symbol.into());
        parameters.insert("period".into(), interval.into());

        if let Some(lt) = limit.into() { parameters.insert("size".into(), format!{"{}", lt});}
        if let Some(st) = start_time.into() { parameters.insert("from".into(), format!("{}", st));}
        if let Some(et) = end_time.into() { parameters.insert("to".into(), format!("{}", et));}

        let data: String = self.get("/swap-ex/market/history/kline", &parameters)?;
        let klines: Klines = from_str(data.as_str())?;

        Ok(klines)
    }

    // Get Merged data
    pub fn get_market_merged<S1>(&self, contract_code: S1) -> APIResult<MergedInfo> 
    where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        let  data: String = self.get("/swap-ex/market/detail/merged", &parameters)?;

        let info: MergedInfo = from_str(data.as_str())?;

        Ok(info)
    }


    //Get market trade
    pub fn get_market_trade<S1>(&self, contract_code: S1) -> APIResult<Trade> 
    where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        let data: String = self.get("/swap-ex/market/trade", &parameters)?;

        let info: Trade = from_str(data.as_str())?;

        Ok(info)
    }

    // Get market history trade
    pub fn get_market_history_trade<S1>(&self, contract_code: S1, size: u32) -> APIResult<HistoryTrade>
    where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        parameters.insert("size".into(), format!{"{}", size});

        let data: String = self.get("/swap-ex/market/history/trade", &parameters)?;

        let info: HistoryTrade = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Risk Info
    pub fn get_risk_info<S1>(&self, contract_code: S1) -> APIResult<RiskInfo> 
    where S1: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(code)= contract_code.into() { parameters.insert("contract_code".into(), code); }

        let data: String = self.get("/swap-api/v1/swap_risk_info", &parameters)?;

        let info: RiskInfo = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Insurance Fund
    pub fn get_insurance_fund<S1>(&self, contract_code: S1) -> APIResult<InsuranceFund>
    where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        let data: String = self.get("/swap-api/v1/swap_insurance_fund", &parameters)?;

        let info: InsuranceFund = from_str(data.as_str())?;

        Ok(info)
    }

    // Get adjust factor
    pub fn get_adjust_factor<S1>(&self, contract_code: S1) -> APIResult<AdjustFactor>
    where S1: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(code) = contract_code.into() { parameters.insert("contract_code".into(), code);}

        let data: String = self.get("/swap-api/v1/swap_adjustfactor", &parameters)?;

        let info: AdjustFactor = from_str(data.as_str())?;

        Ok(info)
    }

    // Get open interest
    pub fn get_his_open_interest<S1, S2, S3>(&self, contract_code: S1, period: S2, size: S3, amount_type: u32) -> APIResult<HisOpenInterest>
    where S1: Into<String>, S2: Into<String>, S3: Into<Option<u32>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        parameters.insert("period".into(), period.into());

        if let Some(sz) = size.into() { parameters.insert("size".into(), format!{"{}", sz});}

        parameters.insert("amount_type".into(), format!{"{}", amount_type});

        let data: String = self.get("/swap-api/v1/swap_his_open_interest", &parameters)?;

        let info: HisOpenInterest =  from_str(data.as_str())?;

        Ok(info)
    }

    // Get elite account ratio
    pub fn get_elite_account_ratio<S1, S2>(&self, contract_code: S1, period: S2) -> APIResult<EliteAccountRatio>
    where S1: Into<String>, S2: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        parameters.insert("period".into(), period.into());

        let data: String = self.get("/swap-api/v1/swap_elite_account_ratio", &parameters)?;

        let info: EliteAccountRatio = from_str(data.as_str())?;

        Ok(info)
    }

    // Get elite account position
    pub fn get_elite_position_ratio<S1, S2>(&self, contract_code: S1, period: S2) -> APIResult<ElitePositionRatio>
    where S1: Into<String>, S2: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        parameters.insert("period".into(), period.into());

        let data: String = self.get("/swap-api/v1/swap_elite_position_ratio", &parameters)?;

        let info: ElitePositionRatio = from_str(data.as_str())?;

        Ok(info)
    }

    // Get api state of system
    pub fn get_api_state<S1>(&self, contract_code: S1) -> APIResult<ApiState>
    where S1: Into<Option<String>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(code) = contract_code.into() { parameters.insert("contract_code".into(), code); }

        let data: String = self.get("/swap-api/v1/swap_api_state", &parameters)?;

        let info: ApiState = from_str(data.as_str())?;

        Ok(info)
    }

    // Get funding rate
    pub fn get_funding_rate<S1>(&self, contract_code: S1) -> APIResult<FundingRate>
    where S1: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());

        let data: String = self.get("/swap-api/v1/swap_funding_rate", &parameters)?;

        let info: FundingRate = from_str(data.as_str())?;

        Ok(info)
    }

    // Get historical funding rate
    pub fn get_his_funding_rate<S1, S2, S3>(&self, contract_code: S1, page_index: S2, page_size: S3) -> APIResult<HisFundingRate>
    where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        if let Some(offset) = page_index.into() { parameters.insert("page_index".into(), format!("{}",offset)); }
        if let Some(limit) = page_size.into() { parameters.insert("page_size".into(), format!("{}",limit)); }

        let data: String = self.get("/swap-api/v1/swap_historical_funding_rate", &parameters)?;

        let info: HisFundingRate = from_str(data.as_str())?;

        Ok(info)
    }

    // Get liquidation orders
    pub fn get_liquidation_orders<S1, S2, S3>(&self, contract_code: S1, trade_type: u32, days: u32, page_index: S2, page_size: S3) -> APIResult<LiquidationOrdersInfo>
    where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("contract_code".into(), contract_code.into());
        parameters.insert("trade_type".into(), format!{"{}", trade_type});
        parameters.insert("create_date".into(), format!{"{}", days});

        if let Some(offset) = page_index.into() { parameters.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { parameters.insert("page_size".into(), format!("{}", limit)); }

        let data: String = self.get("/swap-api/v1/swap_liquidation_orders", &parameters)?;

        let info: LiquidationOrdersInfo = from_str(data.as_str())?;

        Ok(info)
    }

}
