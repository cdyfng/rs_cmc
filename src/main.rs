mod cmc;

use crate::cmc::{CMCClient, CMCListing, CMCListingResponse};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");
    const CMC_KEY: &str = "845c42cb-56b3-483b-a32f-baea0ed5d874";
    let cmc = CMCClient::new(CMC_KEY.to_string());

    loop {
        let now = SystemTime::now();
        let prices = cmc.latest_listings(30);

        let price_map = cmc_listings_as_map(&prices);
        // let price_btc = match price_map.get("BTC") {
        //     Some(price) => match price.quote.get("USD") {
        //         Some(quote) => quote.price,
        //         None => panic!("Could not find BTC price"),
        //     },
        //     None => panic!("Could not find BTC price"),
        // };
        //println!("price_btc {:?}", price_btc);
        println!("{:?}", now);
        for (key, value) in price_map {
            println!("{} / {}", key, value.quote.get("USD").unwrap().price);
            //println!("{} ", key);
            //map.remove(key);
        }
        thread::sleep(Duration::new(1, 0));
    }
}

fn cmc_listings_as_map<'a>(listing: &'a CMCListingResponse) -> HashMap<String, &'a CMCListing> {
    let mut h_map = HashMap::new();
    for l in &listing.data {
        h_map.insert(l.symbol.to_owned(), l);
    }
    h_map
}
