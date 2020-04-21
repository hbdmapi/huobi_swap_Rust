use super::*;
use serde_json::from_str;

impl Client {
    // Get Account Information
    pub fn get_account<S>(&self, symbol: S) -> APIResult<AccountInfo>
        where S: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();
            if let Some(code) = symbol.into() { 
                body.insert("contract_code".into(), format!("{}", code));
            }
            println!("body: {:?}", body.clone());
            let data = self.post_signed("/swap-api/v1/swap_account_info", params, &body)?;

            let account_info: AccountInfo = from_str(data.as_str())?;

            Ok(account_info)
        }
    
    // Get Position Information
    pub fn get_position_info<S>(&self, symbol: S) -> APIResult<PositionInfo>
        where S: Into<Option<String>>
        {
            let mut body:BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();
            if let Some(code) = symbol.into() {
                body.insert("contract_code".into(), format!("{}", code));
            }
            let data = self.post_signed("/swap-api/v1/swap_position_info", params, &body)?;

            let position_info: PositionInfo = from_str(data.as_str())?;

            Ok(position_info)
        }
    
    // get sub-account info by master-account
    pub fn get_sub_account_list<S>(&self, contract_code: S) -> APIResult<SubAccountList> 
        where S: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            let data = self.post_signed("/swap-api/v1/swap_sub_account_list", params, &body)?;

            let subaccountinfo: SubAccountList = from_str(data.as_str())?;

            Ok(subaccountinfo)
        }

    // get sub account info of the sub-uid by master
    pub fn get_sub_account_info<S>(&self, contract_code: S, sub_uid: u64) -> APIResult<SubAccountInfo>
        where S: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            body.insert("sub_uid".to_string(), format!{"{}", sub_uid});

            let data = self.post_signed("/swap-api/v1/swap_sub_account_info", params, &body)?;

            let subaccountinfo: SubAccountInfo = from_str(data.as_str())?;

            Ok(subaccountinfo)
        }

    
    // get sub position info of the sub-uid by master
    pub fn get_sub_position_info<S>(&self, contract_code: S, sub_uid: u64) -> APIResult<SubPositionInfo>
        where S: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            body.insert("sub_uid".to_string(), format!{"{}", sub_uid});

            let data = self.post_signed("/swap-api/v1/swap_sub_position_info", params, &body)?;

            let subpositioninfo: SubPositionInfo = from_str(data.as_str())?;

            Ok(subpositioninfo)
        }
    
    // get financial record
    pub fn get_financial_record<S1, S2, S3, S4, S5>(&self, contract_code: S1, trade_type: S2, created_date: S3, page_index: S4, page_size: S5) -> APIResult<FinancialRecord>
        where S1: Into<String>, S2: Into<Option<String>>, S3: Into<Option<u32>>, S4: Into<Option<u32>>, S5: Into<Option<u32>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            body.insert("contract_code".into(), contract_code.into());

            if let Some(t_type) = trade_type.into() { body.insert("type".into(), format!("{}", t_type));}
            if let Some(c_date) = created_date.into() { body.insert("created_date".into(), format!("{}", c_date)); }
            if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset)); }
            if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

            let data = self.post_signed("/swap-api/v1/swap_financial_record", params, &body)?;

            let financialrecord: FinancialRecord = from_str(data.as_str())?;

            Ok(financialrecord)
        }
   
    // get order limit
    pub fn get_order_limit<S1, S2>(&self, contract_code: S1, order_price_type: S2) -> APIResult<OrderLimitInfo>
        where S1: Into<Option<String>>, S2: Into<String>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            body.insert("order_price_type".into(), order_price_type.into());

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            let data = self.post_signed("/swap-api/v1/swap_order_limit", params, &body)?;

            let orderlimit: OrderLimitInfo = from_str(data.as_str())?;

            Ok(orderlimit)
        }
    
    // get fee info
    pub fn get_fee_info<S1>(&self, contract_code: S1) -> APIResult<FeeInfo>
        where S1: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            let data = self.post_signed("/swap-api/v1/swap_fee", params, &body)?;

            let fee_info: FeeInfo = from_str(data.as_str())?;

            Ok(fee_info)
        }
    
        
    // get transfer limit
    pub fn get_transfer_limit<S1>(&self, contract_code: S1) -> APIResult<TransferLimitInfo>
        where S1: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            let data = self.post_signed("/swap-api/v1/swap_transfer_limit", params, &body)?;

            let transfer_limit: TransferLimitInfo = from_str(data.as_str())?;

            Ok(transfer_limit)
        }
    
    // get position limit
    pub fn get_position_limit<S1>(&self, contract_code: S1) -> APIResult<PositionLimitInfo>
        where S1: Into<Option<String>>
        {
            let mut body: BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            if let Some(code) = contract_code.into() { body.insert("contract_code".into(), format!("{}", code));}

            let data = self.post_signed("/swap-api/v1/swap_position_limit", params, &body)?;

            let position_limit: PositionLimitInfo = from_str(data.as_str())?;

            Ok(position_limit)
        }
    
    // transfer between master and sub account 
    pub fn swap_master_sub_transfer(&self, sub_uid: String, contract_code: String, amount: f64, transfer_type: String) -> APIResult<MasterSubTransferInfo>
    {
        let mut body: BTreeMap<String, String> = BTreeMap::new();
        let params: BTreeMap<String, String> = BTreeMap::new();
        
        body.insert("sub_uid".into(), sub_uid.to_string());
        body.insert("contract_code".into(), contract_code.to_string());
        body.insert("amount".into(), amount.to_string());
        body.insert("type".into(), transfer_type.to_string());

        let data = self.post_signed("/swap-api/v1/swap_master_sub_transfer", params, &body)?;

        let transfer_info: MasterSubTransferInfo = from_str(data.as_str())?;

        Ok(transfer_info)
    }
    
    // query swap master and sub account records
    pub fn get_master_sub_transfer_record<S1, S2, S3, S4>(&self, contract_code: S1, transfer_type: S2, days: u32, page_index: S3, page_size: S4) -> APIResult<MasterSubTransferRecordInfo>
    where S1: Into<String>, S2: Into<Option<String>>, S3: Into<Option<u32>>, S4: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".to_string(), contract_code.into());
        if let Some(trans_type) = transfer_type.into() { body.insert("transfer_type".into(), format!("{}", trans_type)); }
        if let Some(day) = days.into() { body.insert("create_date".into(), format!("{}", day));}
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/swap-api/v1/swap_master_sub_transfer_record", params, &body)?;

        let transfer_record: MasterSubTransferRecordInfo = from_str(data.as_str())?;

        Ok(transfer_record)

    }

    // query api status
    pub fn get_api_trading_status(&self) -> APIResult<ApiTradeStatus>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();

        let data = self.get_signed("/swap-api/v1/swap_api_trading_status", params)?;

        let api_status: ApiTradeStatus = from_str(data.as_str())?;

        Ok(api_status)
    }  

    // place order
    pub fn place_order<S1, S2, S3, S5, S6, S8>(&self, contract_code: S1, client_order_id: S2, price: S3, volume: u32,
                                direction: S5, offset: S6, lever_rate: u32, order_price_type: S8) -> APIResult<OrderInfo>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<f64>>, S5: Into<String>, S6: Into<String>, 
        S8: Into<String>
        {
            let mut body:BTreeMap<String, String> = BTreeMap::new();
            let params: BTreeMap<String, String> = BTreeMap::new();

            body.insert("contract_code".into(), contract_code.into());
            body.insert("volume".into(), format!("{}", volume));
            body.insert("direction".into(), direction.into());
            body.insert("offset".into(), offset.into());
            body.insert("lever_rate".into(), lever_rate.to_string());
            body.insert("order_price_type".into(), order_price_type.into());

            if let Some(client_id) = client_order_id.into() { body.insert("client_order_id".into(), format!("{}", client_id)); }
            if let Some(p) = price.into() { body.insert("price".into(), format!("{}", p)); }
            
            println!("body: {:?}", body.clone());

            let data = self.post_signed("/swap-api/v1/swap_order", params, &body)?;

            let order: OrderInfo = from_str(data.as_str())?;

            Ok(order)

        }
    
    // place batch order
    pub fn place_orders(&self, orders_data: BatchOrderRequest) -> APIResult<BatchOrder> 
    {
        let params: BTreeMap<String, String> = BTreeMap::new();

        let data = self.post_signed("/swap-api/v1/swap_batchorder", params, &orders_data)?;

        let order: BatchOrder = from_str(data.as_str())?;

        Ok(order)
    } 

    // cancel orders
    pub fn cancel_orders<S1, S2, S3>(&self, order_id: S1, client_order_id: S2, contract_code: S3) -> APIResult<OrderCancelInfo>
    where S1: Into<Option<String>>, S2: Into<Option<String>>, S3: Into<String>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());

        if let Some(oid) = order_id.into() {body.insert("order_id".into(), format!("{}", oid)); }
        if let Some(cid) = client_order_id.into() { body.insert("client_order_id".into(), cid); }

        let data = self.post_signed("/swap-api/v1/swap_cancel", params, &body)?;

        let cancel: OrderCancelInfo = from_str(data.as_str())?;

        Ok(cancel)
    }

    // cancel all orders
    pub fn cancel_all(&self, contract_code: String) -> APIResult<OrderCancelInfo> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".to_string(), contract_code);

        let data = self.post_signed("/swap-api/v1/swap_cancelall", params, &body)?;

        let cancel_all: OrderCancelInfo = from_str(data.as_str())?;

        Ok(cancel_all)
    }

    // get order info
    pub fn get_order_info<S1, S2, S3>(&self, order_id: S1, client_order_id: S2, contract_code: S3) -> APIResult<GOrderInfo> 
        where S1: Into<Option<String>>, S2: Into<Option<String>>, S3: Into<String>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        if let Some(oid) = order_id.into() { body.insert("order_id".into(), format!("{}", oid));}
        if let Some(cid) = client_order_id.into() { body.insert("client_order_id".into(), format!("{}", cid));}

        let data = self.post_signed("/swap-api/v1/swap_order_info", params, &body)?;

        let order_info: GOrderInfo = from_str(data.as_str())?;

        Ok(order_info)
    }

    // get order detail information
    pub fn get_order_detail<S1, S2, S3, S4>(&self, contract_code: S1, order_id: u64, created_at: S2, order_type: u32, 
        page_index: S3, page_size: S4) -> APIResult<OrderDetailInfo> 
        where S1: Into<String>, S2: Into<Option<u64>>, S3: Into<Option<u32>>, S4: Into<Option<u32>>
        {
            let params: BTreeMap<String, String> = BTreeMap::new();
            let mut body: BTreeMap<String, String> = BTreeMap::new();

            body.insert("contract_code".into(), contract_code.into());
            body.insert("order_id".into(), format!("{}", order_id));
            body.insert("order_type".to_string(), format!("{}", order_type));
            if let Some(ct) = created_at.into() { body.insert("created_at".into(),format!("{}", ct));}
            if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
            if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

            let data = self.post_signed("/swap-api/v1/swap_order_detail", params, &body)?;

            let order_detail: OrderDetailInfo = from_str(data.as_str())?;

            Ok(order_detail)

        }
    // get open orders
    pub fn get_open_orders<S1, S2, S3>(&self, contract_code: S1, page_index: S2, page_size: S3) -> APIResult<OpenOrders>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
        {
            let params: BTreeMap<String, String> = BTreeMap::new();
            let mut body: BTreeMap<String, String> = BTreeMap::new();

            body.insert("contract_code".into(), contract_code.into());
            if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
            if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

            let data = self.post_signed("/swap-api/v1/swap_openorders", params, &body)?;

            let open_orders: OpenOrders = from_str(data.as_str())?;

            Ok(open_orders)

        }
    
    // get history orders
    pub fn get_his_orders<S1, S2, S3>(&self, contract_code: S1, trade_type: u32, r_type: u32, status: String, create_date: u32, page_index: S2, page_size: S3)
    -> APIResult<HisOrders>
    where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        body.insert("trade_type".into(), format!("{}", trade_type));
        body.insert("type".into(), format!("{}", r_type));
        body.insert("status".into(), format!("{}", status));
        body.insert("create_date".into(), format!("{}", create_date));
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/swap-api/v1/swap_hisorders", params, &body)?;

        let his_orders: HisOrders = from_str(data.as_str())?;

        Ok(his_orders)

    }
     
    // get match results
    pub fn get_match_results<S1, S2, S3>(&self, contract_code: S1, trade_type: u32, days: u32, page_index: S2, page_size: S3)
       -> APIResult<MatchResults>
       where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
       {
           let params: BTreeMap<String, String> = BTreeMap::new();
           let mut body: BTreeMap<String, String> = BTreeMap::new();
   
           body.insert("contract_code".into(), contract_code.into());
           body.insert("trade_type".into(), format!("{}", trade_type));
           body.insert("create_date".into(), format!("{}", days));
           if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
           if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}
   
           let data = self.post_signed("/swap-api/v1/swap_matchresults", params, &body)?;
   
           let match_results: MatchResults = from_str(data.as_str())?;
   
           Ok(match_results)
       }
    
    // lightning close
    pub fn lightning_close<S1, S2, S3>(&self, contract_code: S1, volume: u32, direction: String, client_order_id: S2, order_price_type: S3) -> APIResult<LightningCloseResult>
       where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
       {
           let params: BTreeMap<String, String> = BTreeMap::new();
           let mut body: BTreeMap<String, String> = BTreeMap::new();

           body.insert("contract_code".into(), contract_code.into());
           body.insert("volume".into(), format!("{}", volume));
           body.insert("direction".into(), format!("{}", direction));
           
           if let Some(cid) = client_order_id.into() { body.insert("client_order_id".into(), format!("{}", cid)); }
           if let Some(otype) = order_price_type.into() { body.insert("order_price_type".into(), format!("{}", otype)); }

           let data = self.post_signed("/swap-api/v1/swap_lightning_close_position", params, &body)?;

           let result: LightningCloseResult = from_str(data.as_str())?;

           Ok(result)

       }
    
    pub fn spot_account_transfer<S1, S2, S3>(&self, from: S1, to: S2, currency: S3, amount: f64) -> APIResult<AccountTransferResult>
       where S1: Into<String>, S2: Into<String>, S3: Into<String>
       {
           let params: BTreeMap<String, String> = BTreeMap::new();
           let mut body: BTreeMap<String, String> = BTreeMap::new();

           body.insert("from".into(), from.into());
           body.insert("to".into(), to.into());
           body.insert("currency".into(), currency.into());
           body.insert("amount".into(), format!("{}", amount));

           let data = self.post_signed("/v2/account/transfer", params, &body)?;

           let account_transfer: AccountTransferResult = from_str(data.as_str())?;

           Ok(account_transfer)
       }

    // pub fn balance(&self, id: u32) -> APIResult<Balance> {
        // let params: BTreeMap<String, String> = BTreeMap::new();
        // let data = self.get_signed(&format!("/v1/account/accounts/{}/balance", id), params)?;
        // let response: APIResponse<Balance> = from_str(data.as_str())?;
        // Ok(response.data)
    // }

    // pub fn orders(&self, symbol: &str, states: &str) -> APIResult<Vec<Order>> {
        // let mut params: BTreeMap<String, String> = BTreeMap::new();
        // params.insert("symbol".to_string(), symbol.to_string());
        // params.insert(
            // "states".to_string(),
            // states.to_string(),
        // );
//        params.insert("types".to_string(), "buy-limit".to_string());
        // let data = self.get_signed("/v1/order/orders", params)?;
        // let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;

        // Ok(response.data)
    // }
}
