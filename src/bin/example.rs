
extern crate orderbook;

use std::time::SystemTime;
use orderbook::{Orderbook, OrderSide, orders};


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    INR,
    CAD,
    BTC,
}


fn parse_asset(asset: &str) -> Option<BrokerAsset> {
    match asset {
        "INR" => Some(BrokerAsset::INR),
        "CAD" => Some(BrokerAsset::CAD),
        "BTC" => Some(BrokerAsset::BTC),    
        _ => None,
    }
}


fn main() {
    let mut orderbook = Orderbook::new(BrokerAsset::BTC, BrokerAsset::INR);
    let order_asset = parse_asset("BTC").unwrap();
    let price_asset = parse_asset("INR").unwrap();

    // create order requests
    let order_list =
        vec![
            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Bid,
                2_443_830.00,
                5.0,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Ask,
                2_474_674.00,
                1.0,
                SystemTime::now()
            ),

            orders::amend_order_request(1, OrderSide::Bid, 2_450_035.00, 4.0, SystemTime::now()),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Bid,
                2_444_830.00,
                0.4,
                SystemTime::now()
            ),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Ask,
                2_496_028.00,
                0.5,
                SystemTime::now()
            ),

            orders::new_market_order_request(order_asset, price_asset, OrderSide::Bid, 1.0, SystemTime::now()),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Ask,
                2_498_915.00,
                0.5,
                SystemTime::now()
            ),

            orders::limit_order_cancel_request(4, OrderSide::Ask),

            orders::new_limit_order_request(
                order_asset,
                price_asset,
                OrderSide::Bid,
                2_431_000.00,
                0.6,
                SystemTime::now()
            ),
        ];

    // processing
    for order in order_list {
        println!("Order => {:?}", &order);
        let res = orderbook.process_order(order);
        println!("Processing => {:?}", res);
        if let Some((bid, ask)) = orderbook.current_spread() {
            println!("Spread => bid: {}, ask: {}\n", bid, ask);
        } else {
            println!("Spread => not available\n");
        }
    }
}
